use std::{
    ffi::OsStr,
    io::{self, ErrorKind},
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use futures_util::future::OptionFuture;
use sys_mount::{MountFlags, SupportedFilesystems};
use tokio::{
    self,
    fs::{File, OpenOptions},
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    task::{self, JoinHandle},
    time::timeout,
};

use crate::{
    error::{
        AgentLifecycleError, AgentShutdownError, HandleMessageError, HandleMountsError,
        ReceieveAgentError, TakeFileError,
    },
    mount::{self, MountRequest},
    stream::{MessageReader, Stream},
    ExecutionConfig,
};

use self::agent::RunningAgent;

mod agent;
pub mod spawn;

/// The UID of the unprivileged DOXA user whose home directory is `/home/doxa`
pub const DOXA_UID: u32 = 1000;
pub const DOXA_GID: u32 = 1000;

/// An upper bound on the agent tar size for sanity reasons, measured in bytes
pub const MAX_AGENT_SIZE: usize = 3_000_000_000;
/// Maximum length for messages other than the agent file in bytes
pub const MAX_MSG_LEN: usize = 50_000_000;
pub const MAX_FILE_NAME_LEN: usize = 300;
pub const STDERR_LEN: usize = 100_000;

/// This is the server that runs inside of the VM.
pub struct VMExecutor<S: AsyncWrite + AsyncRead + Unpin + Send + 'static> {
    stream: Stream<S>,
    execution_root: PathBuf,
    execution_config: ExecutionConfig,
    agent: Option<RunningAgent>,
}

impl<S: AsyncWrite + AsyncRead + Unpin + Send + 'static> VMExecutor<S> {
    pub fn start(stream: S) -> JoinHandle<()> {
        task::spawn(async move {
            let mut stream = Stream::from_socket(stream);

            Self::handle_mounts(&mut stream).await.unwrap();

            let output_dir = PathBuf::from_str("/home/doxa/agent").unwrap();
            tokio::fs::create_dir_all(&output_dir).await.unwrap();

            Self::receive_agent(&mut stream, &output_dir)
                .await
                .expect("Failed to receive agent");
            println!("Received agent");

            // TODO: better reporting of errors
            let (config_dir, mut config_file) = Self::find_config_dir(output_dir)
                .await
                .expect("Couldn't find config dir/file");
            let mut config = String::with_capacity(1000);
            config_file.read_to_string(&mut config).await.unwrap();

            let config: ExecutionConfig =
                serde_yaml::from_str(&config).expect("Invalid config file");

            let mut executor = VMExecutor {
                stream,
                execution_root: config_dir,
                execution_config: config,
                agent: None,
            };

            let mut message_reader = MessageReader::new(Vec::new(), MAX_MSG_LEN);

            // Change next_full_message to return a struct that impl's future and is cancellable
            loop {
                tokio::select! {
                    Some(result) = OptionFuture::from(executor.agent.as_mut().map(|agent| agent.next_line())) => {
                        match result.unwrap() {
                            Some(line) => executor.handle_output_line(line).await.unwrap(),
                            // Agent proecss finished
                            None => executor.handle_agent_terminated().await.unwrap(),
                        }
                    }
                    message = message_reader.read_full_message(&mut executor.stream) => {
                        let message = message.expect("failed to read message");
                        println!("Got line {}", String::from_utf8_lossy(message));
                        executor.handle_message(message).await.unwrap();
                    }
                };
            }
        })
    }

    async fn handle_agent_terminated(&mut self) -> io::Result<()> {
        println!("Agent terminated");
        let mut err_output = String::new();
        if let Some(mut agent) = self.agent.take() {
            // NOTE: currently STDERR get's stored in memory until the agent exits.
            agent
                .child_process
                .stderr
                .take()
                .unwrap()
                .take(STDERR_LEN as u64)
                .read_to_string(&mut err_output)
                .await?;
        }

        self.stream
            .send_prefixed_full_message(b"F_", err_output.as_bytes())
            .await?;

        Ok(())
    }

    async fn handle_output_line(&mut self, line: String) -> io::Result<()> {
        // Send the line across the stream
        self.stream
            .send_prefixed_full_message(b"OUTPUT_", line.as_bytes())
            .await
    }

    async fn handle_message(&mut self, msg: &[u8]) -> Result<(), HandleMessageError> {
        let split_location = msg
            .iter()
            .position(|b| *b == b'_')
            .ok_or(HandleMessageError::MissingSeparator)?;
        let (prefix, msg) = msg.split_at(split_location);
        // Exclude the _ character itself
        let msg = &msg[1..];

        match prefix {
            b"INPUT" => {
                if let Some(agent) = self.agent.as_mut() {
                    // This could happen legitimately if the agent crashes between when we detect
                    // it
                    if let Err(e) = agent.stdin.write_all(msg).await {
                        println!("failed to send input to agent due to: {}", e);
                    }
                } else {
                    println!("Tried to send input to dead agent (ignoring)");
                }
            }
            // This may not be very useful, there isn't really a good reason to do this
            b"SHUTDOWN" => self.shutdown(true).await?,
            b"SPAWN" => self.spawn(msg).await?,
            b"REBOOT" => self.reboot(msg).await?,
            b"TAKEFILE" => self.take_file(msg).await?,
            _ => return Err(HandleMessageError::UnrecognisedPrefix),
        }

        Ok(())
    }

    async fn take_file(&mut self, msg: &[u8]) -> Result<(), TakeFileError> {
        let path = PathBuf::from(OsStr::from_bytes(msg));
        println!("take file {:?}", path);

        let metadata = tokio::fs::metadata(&path)
            .await
            .map_err(|e| match e.kind() {
                io::ErrorKind::NotFound => TakeFileError::FileNotFound,
                _ => e.into(),
            })?;

        if !metadata.is_file() {
            return Err(TakeFileError::NotFile);
        }

        // In future this could be relaxed, each competition could specify a MAX file size and if a
        // file exceeds the MAX_MSG_LEN but not the max file size, then it could be sent in chunks.
        if metadata.len() >= MAX_MSG_LEN as u64 {
            return Err(TakeFileError::FileTooLarge);
        }

        let file = tokio::fs::read(&path).await?;
        tokio::fs::remove_file(&path).await?;

        self.stream
            .send_prefixed_full_message(b"FILEDATA_", &file)
            .await?;

        Ok(())
    }

    async fn shutdown(&mut self, required: bool) -> Result<(), AgentLifecycleError> {
        if let Some(agent) = self.agent.as_mut() {
            agent
                .child_process
                .kill()
                .await
                .map_err(AgentShutdownError::FailedToKillAgent)?;
        } else if required {
            return Err(AgentShutdownError::AgentNotRunning.into());
        }

        self.stream.send_full_message(b"SHUTDOWN").await.unwrap();

        Ok(())
    }

    async fn spawn(&mut self, arg_msg: &[u8]) -> Result<(), AgentLifecycleError> {
        // Format of args is \0{arg_1}\0{arg_2} (i.e. \0 then arg, if no args then empty)
        let args: Vec<_> = arg_msg
            .split(|b| *b == b'\0')
            .skip(1)
            .map(OsStr::from_bytes)
            .collect();

        self.agent =
            Some(RunningAgent::spawn(&self.execution_config, &self.execution_root, args).await?);

        self.stream.send_full_message(b"SPAWNED").await.unwrap();

        Ok(())
    }

    async fn reboot(&mut self, arg_msg: &[u8]) -> Result<(), AgentLifecycleError> {
        // Reboot doesn't require that an agent was previously running
        self.shutdown(false).await?;

        self.spawn(arg_msg).await?;

        Ok(())
    }

    async fn handle_mounts(stream: &mut Stream<S>) -> Result<(), HandleMountsError> {
        let mut mount_msg = Vec::with_capacity(500);
        stream
            .next_full_message(&mut mount_msg, MAX_MSG_LEN)
            .await?;

        let (prefix, mount_request) = mount_msg.split_at(
            mount_msg
                .iter()
                .position(|b| *b == b'_')
                .ok_or(HandleMountsError::InvalidFormatting)?,
        );

        if prefix == b"NOMOUNTREQUEST" {
            println!("Got NOMOUNTREQUEST skipping mounting");
            return Ok(());
        }

        if prefix != b"MOUNTREQUEST" {
            println!(
                "Did not get expected prefix 'MOUNTREQUEST' got '{}'",
                String::from_utf8_lossy(prefix)
            );
            return Err(HandleMountsError::InvalidFormatting);
        }

        // Skip the '_'
        let mount_request = &mount_request[1..];
        let mount_request: MountRequest =
            serde_json::from_slice(mount_request).expect("Failed to deserialize mount request");

        let drives = mount::find_drives_by_uuid()
            .await
            .map_err(HandleMountsError::FindDrives)?;

        let mut swap_msg = mount_msg;
        stream.next_full_message(&mut swap_msg, MAX_MSG_LEN).await?;

        if swap_msg == b"SWAPON" {
            mount::swapon()
                .await
                .map_err(HandleMountsError::ActivateSwap)?;
        } else if swap_msg != b"NO SWAP" {
            return Err(HandleMountsError::InvalidSwapMessage);
        }

        let supported = SupportedFilesystems::new().expect("failed to get supported file systems");

        for (uuid, path_on_guest, read_only) in mount_request.mounts {
            let drive = drives
                .get(&uuid)
                .ok_or_else(|| HandleMountsError::UUIDNotFound {
                    uuid,
                    mount_path: path_on_guest.clone(),
                })?;

            let mount_flags = if read_only {
                MountFlags::RDONLY
            } else {
                MountFlags::empty()
            };

            println!("Mounting {} to {}", drive, path_on_guest);
            tokio::fs::create_dir_all(&path_on_guest).await.unwrap();
            sys_mount::Mount::new(drive, path_on_guest, &supported, mount_flags, None)?;
        }

        stream.send_full_message(b"MOUNTED").await?;

        Ok(())
    }

    /// Download the agent to `{output_dir}/download/agent_name.tar[.gz]`
    /// Then extract the tar file to `{output_dir}` and delete the downloaded tar.
    async fn receive_agent(
        stream: &mut Stream<S>,
        output_dir: &Path,
    ) -> Result<(), ReceieveAgentError> {
        // == Name message
        let mut name_msg = Vec::with_capacity(100);
        stream
            .next_full_message(&mut name_msg, MAX_FILE_NAME_LEN)
            .await?;

        // N for Name
        if name_msg[0] != b'N' {
            println!("Invalid message char {} for name", name_msg[0]);
            return Err(ReceieveAgentError::InvalidFormatting);
        }

        let name = String::from_utf8_lossy(&name_msg[1..]);

        let download_location = output_dir.join("download").join(name.as_ref());

        tokio::fs::create_dir_all(&output_dir.join("download")).await?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&download_location)
            .await?;

        println!("Beginning download of agent {}", name);
        // == File data
        let file_len = stream
            .next_message_to_writer(&mut file, MAX_AGENT_SIZE)
            .await?;

        println!("Downloaded {} bytes", file_len);

        timeout(
            Duration::from_secs(10),
            stream.expect_exact_msg(b"FILE ENDS"),
        )
        .await
        .map_err(|_| ReceieveAgentError::Timeout {
            during: "wait for `FILE ENDS`".to_string(),
        })??;

        println!("FILE ENDS");

        let mut tar_process = tokio::process::Command::new("tar")
            .args(&[
                "xf",
                download_location.to_str().unwrap(),
                &format!("--directory={}", output_dir.to_str().unwrap()),
            ])
            .uid(DOXA_UID)
            .gid(DOXA_GID)
            .spawn()
            .expect("Couldn't spawn tar");

        let status = timeout(Duration::from_secs(60 * 2), tar_process.wait())
            .await
            .map_err(|_| ReceieveAgentError::Timeout {
                during: "tar extraction".to_string(),
            })??;

        if !status.success() {
            return Err(ReceieveAgentError::ExtractError);
        }

        stream.send_full_message(b"RECEIVED").await?;

        tokio::fs::remove_file(download_location)
            .await
            .expect("Couldn't delete agent tar");

        Ok(())
    }

    /// Returns the directory containing the config (`doxa.yaml`) file and the file pointer itself.
    /// At each folder it will check to see if the config file exists.
    /// If there is only a single folder in the directory it will recurse downwards which may be
    /// necessary depending on how the tar file was created.
    async fn find_config_dir(agent_dir: PathBuf) -> io::Result<(PathBuf, File)> {
        let mut search_path = agent_dir;

        loop {
            match OpenOptions::new()
                .read(true)
                .open(search_path.join("doxa.yaml"))
                .await
            {
                Ok(file) => return Ok((search_path, file)),
                Err(e) if e.kind() == ErrorKind::NotFound => {}
                Err(e) => return Err(e),
            }

            let mut entries = tokio::fs::read_dir(&search_path).await?;

            let mut dir = None;
            let mut single_dir = true;

            while let Some(entry) = entries.next_entry().await? {
                if let Ok(filetype) = entry.file_type().await {
                    if filetype.is_dir() {
                        if dir == None {
                            dir = Some(entry.path());
                        } else {
                            single_dir = false;
                        }
                    }
                }
            }

            if let (Some(dir), true) = (dir, single_dir) {
                // Recurse down
                search_path = dir;
            } else {
                // We can no longer continue going down directories as there are multiple, or there are
                // none
                return Err(ErrorKind::NotFound.into());
            }
        }
    }
}
