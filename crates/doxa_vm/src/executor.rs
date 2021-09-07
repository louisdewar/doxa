use std::{
    io::{self, ErrorKind},
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use tokio::{
    self,
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{self, Child, ChildStdin},
    task::{self, JoinHandle},
    time::timeout,
};

use tokio_vsock::VsockStream;

use crate::{
    error::{ExecutionSpawnError, HandleMessageError, ReceieveAgentError},
    stream::{MessageReader, Stream},
    ExecutionConfig,
};

mod spawn;

/// The UID of the unprivileged DOXA user whose home directory is `/home/doxa`
pub const DOXA_UID: u32 = 1000;
pub const DOXA_GID: u32 = 1000;

/// An upper bound on the agent size for sanity reasons, measured in bytes
pub const MAX_AGENT_SIZE: usize = 50_000_000;
/// Maximum length for messages other than the agent file in bytes
pub const MAX_MSG_LEN: usize = 5_000;
pub const MAX_FILE_NAME_LEN: usize = 300;

/// This is the server that runs inside of the VM.
pub struct VMExecutor {
    child_process: Child,
    stdin: ChildStdin,
    stream: Stream<VsockStream>,
}

impl VMExecutor {
    pub fn start(cid: u32, port: u32) -> JoinHandle<()> {
        task::spawn(async move {
            let stream = VsockStream::connect(cid, port)
                .await
                .expect("Couldn't connect to stream");

            println!("VM executor connected");
            let mut stream = Stream::from_socket(stream);
            let output_dir = PathBuf::from_str("/tmp/doxa_executor").unwrap();
            tokio::fs::create_dir_all(&output_dir).await.unwrap();

            Self::receive_agent(&mut stream, &output_dir)
                .await
                .expect("Failed to receive agent");

            // TODO: better reporting of errors
            let (config_dir, mut config_file) = Self::find_config_dir(output_dir.join("agent"))
                .await
                .expect("Couldn't find config dir/file");
            let mut config = String::with_capacity(1000);
            config_file.read_to_string(&mut config).await.unwrap();

            let config: ExecutionConfig =
                serde_yaml::from_str(&config).expect("Invalid config file");

            // TODO: canonicalise entrypoint and check it's below /home/doxa

            let mut child_process = Self::spawn(&config, &config_dir)
                .await
                .expect("Failed to spawn agent");

            let stdout = child_process.stdout.take().unwrap();
            let stdin = child_process.stdin.take().unwrap();

            stream.send_full_message(b"SPAWNED").await.unwrap();

            let mut stdout_lines = BufReader::new(stdout).lines();

            let mut executor = VMExecutor {
                child_process,
                stream,
                stdin,
            };

            let mut message_reader = MessageReader::new(Vec::new(), MAX_MSG_LEN);

            // Change next_full_message to return a struct that impl's future and is cancellable
            loop {
                tokio::select! {
                    line = stdout_lines.next_line() => {
                        match dbg!(line.unwrap()) {
                            Some(line) => executor.handle_output_line(line).await.unwrap(),
                            None => break,
                        }
                    }
                    message = message_reader.read_full_message(&mut executor.stream) => {
                        let message = message.expect("failed to read message");
                        println!("Got line {}", String::from_utf8_lossy(&message).to_string());
                        executor.handle_message(message).await.unwrap();
                    }

                };
            }

            // The proceses finished
            executor.stream.send_full_message(b"F_").await.unwrap();
        })
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
            b"INPUT" => self.stdin.write_all(msg).await?,
            _ => return Err(HandleMessageError::UnrecognisedPrefix),
        }

        Ok(())
    }

    /// The root is the directory of the config.
    async fn spawn(config: &ExecutionConfig, root: &Path) -> Result<Child, ExecutionSpawnError> {
        match &config.language {
            &crate::Language::Python => {
                spawn::spawn_python(root, &config.options, &config.entrypoint).await
            }
            lang => todo!("language {:?}", lang),
        }
    }

    /// Download the agent to `{output_dir}/download/agent_name.tar[.gz]`
    /// Then extract the tar file to `{output_dir}/agent` and delete the downloaded tar.
    async fn receive_agent(
        stream: &mut Stream<VsockStream>,
        output_dir: &PathBuf,
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

        let agent_output = output_dir.join("agent");
        tokio::fs::create_dir_all(&agent_output).await?;

        let mut tar_process = process::Command::new("tar")
            .args(&[
                "xf",
                download_location.to_str().unwrap(),
                &format!("--directory={}", agent_output.to_str().unwrap()),
            ])
            .spawn()
            .expect("Couldn't spawn tar");

        let status = timeout(Duration::from_secs(60), tar_process.wait())
            .await
            .map_err(|_| ReceieveAgentError::Timeout {
                during: "tar extraction".to_string(),
            })??;

        if !status.success() {
            return Err(ReceieveAgentError::ExtractError);
        }

        stream.send_full_message(b"RECEIVED").await?;

        Ok(())
    }

    /// Returns the directory containing the config (`doxa.toml`) file and the file pointer itself.
    /// At each folder it will check to see if the config file exists.
    /// If there is only a single folder in the directory it will recurse downwards which may be
    /// necessary depending on how the tar file was created.
    async fn find_config_dir(agent_dir: PathBuf) -> io::Result<(PathBuf, File)> {
        let mut search_path = agent_dir;

        loop {
            match OpenOptions::new()
                .read(true)
                .open(search_path.join("doxa.toml"))
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
