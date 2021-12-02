use crate::error::{AgentLifecycleManagerError, TakeFileManagerError};
use crate::manager::ManagerError::TimeoutWaitingForVMConnection;
use std::time::Duration;
use std::{io, path::PathBuf};
use tokio::time::timeout;

use doxa_firecracker_sdk::{error::ShutdownError, VMOptions, VM};
use tokio::{
    net::{UnixListener, UnixStream},
    task,
};

use tracing::trace;

use crate::{
    error::{ManagerError, SendAgentError},
    executor::MAX_MSG_LEN,
    stream::Stream,
};

use tempfile::{tempdir, TempDir};

/// Manages lifecycle of and communciation with a single VM.
pub struct Manager {
    vm: VM,
    tempdir: TempDir,
    stream: Stream<UnixStream>,
    // stdout: ChildStdout,
    // stderr: ChildStderr,
}

impl Manager {
    /// Spawns up the firecracker process and waits for the process to connect.
    /// This will copy the rootfs to a tempdir so that the VM is allowed write permissions.
    /// In future it will be a good idea to figure out properly how to make a rootfs that can be
    /// booted with read_only access into RAM.
    pub async fn new(
        original_rootfs: PathBuf,
        kernel_img: PathBuf,
        kernel_boot_args: String,
        firecracker_path: PathBuf,
        memory_size_mib: usize,
    ) -> Result<Self, ManagerError> {
        // TODO: consider that when Drop is called for the tempdir by default it will be blocking,
        // maybe implement a custom Drop for manager that calls spawn_blocking?
        // Also if the executor is typically run as it's own process in future it may not matter,
        // it will probably only be a problem if it's being run on the main webserver process.
        let dir = task::spawn_blocking(tempdir).await??;

        let rootfs_path = dir.path().join("rootfs");

        tokio::fs::copy(original_rootfs, &rootfs_path).await?;

        let vm = VMOptions {
            memory_size_mib,
            vcpus: 1,
            kernel_image_path: kernel_img,
            kernel_boot_args,
            rootfs_path,
            rootfs_read_only: false,
            socket: dir.path().join("socket"),
        }
        .spawn(firecracker_path)
        .await?;

        // Begin listening for connections on port 1001
        let listener = UnixListener::bind(dir.path().join("v.sock_1001")).unwrap();

        vm.create_vsock(
            "1".to_string(),
            3,
            dir.path().join("v.sock").to_string_lossy().to_string(),
        )
        .await?;

        vm.instance_start().await?;

        let (stream, _addr) = timeout(Duration::from_secs(30), listener.accept())
            .await
            .map_err(|_| TimeoutWaitingForVMConnection)??;

        let stream = Stream::from_socket(stream);

        // let stdout = vm.firecracker_process().stdout.take().unwrap();
        // let stderr = vm.firecracker_process().stderr.take().unwrap();

        Ok(Manager {
            vm,
            tempdir: dir,
            stream,
            // stdout,
            // stderr,
        })
    }

    /// Sends the agent and then waits for the VM to spawn it
    pub async fn send_agent<S: futures_util::Stream<Item = Result<bytes::Bytes, E>> + Unpin, E>(
        &mut self,
        agent_name: &str,
        agent_size: u64,
        mut agent: S,
    ) -> Result<(), SendAgentError<E>> {
        self.stream
            .send_full_message(format!("N{}", agent_name).as_bytes())
            .await?;

        self.stream
            .send_stream(&mut agent, agent_size as usize)
            .await
            .map_err(SendAgentError::DownloadAgentError)?;

        self.stream
            .send_full_message("FILE ENDS".as_bytes())
            .await?;

        self.stream.expect_exact_msg(b"RECEIVED").await?;
        trace!("VM has received agent");

        //self.stream.expect_exact_msg(b"SPAWNED").await?;
        //info!("VM has spawned agent");

        Ok(())
    }

    /// Sends input to the agent (this will be sent directly to the agent's STDIN).
    /// This will not append a new line this must be provided in the input message itself.
    /// It is possible to send part of a line, a full line or multiple lines with this method.
    pub async fn send_agent_input(&mut self, line: &[u8]) -> Result<(), io::Error> {
        self.stream
            .send_prefixed_full_message(b"INPUT_", line)
            .await
    }

    fn map_args(args: Vec<String>) -> Vec<u8> {
        args.into_iter()
            .map(|arg| {
                let mut v = Vec::with_capacity(1 + arg.len());
                v.push(b'\0');
                v.extend(arg.as_bytes());
                v
            })
            .flatten()
            .collect()
    }

    pub async fn spawn_agent(&mut self, args: Vec<String>) -> Result<(), io::Error> {
        self.stream
            .send_prefixed_full_message(b"SPAWN_", Self::map_args(args).as_ref())
            .await?;

        Ok(())
    }

    /// Sends a reboot message to the VM instructing it to restart the agent's process inside the
    /// VM and waits for the response.
    /// This will discard all other messages until it receives the `SHUTDOWN` and then `SPAWNED` messages from the VM.
    ///
    /// If an agent is not currently running then this is equivalent to `spawn_agent`
    pub async fn reboot_agent(
        &mut self,
        args: Vec<String>,
    ) -> Result<(), AgentLifecycleManagerError> {
        self.stream
            .send_prefixed_full_message(b"REBOOT_", Self::map_args(args).as_ref())
            .await?;

        timeout(Duration::from_secs(20), async {
            // The previous agent may have outputted messages that we haven't read yet, we don't
            // care about them
            let mut buf = Vec::new();

            loop {
                self.stream.next_full_message(&mut buf, MAX_MSG_LEN).await?;

                if &buf == b"SHUTDOWN" {
                    break;
                }
            }

            self.stream.next_full_message(&mut buf, MAX_MSG_LEN).await?;

            if &buf != b"SPAWNED" {
                return Err(AgentLifecycleManagerError::MissingSpawnedMessage);
            }

            Result::<(), AgentLifecycleManagerError>::Ok(())
        })
        .await
        .map_err(|_| AgentLifecycleManagerError::Timeout)??;

        Ok(())
    }

    pub async fn take_file(&mut self, path: String) -> Result<Vec<u8>, TakeFileManagerError> {
        self.stream
            .send_prefixed_full_message(b"TAKEFILE_", path.as_bytes())
            .await?;

        let mut buf = timeout(Duration::from_secs(120), async {
            // The previous agent may have outputted messages that we haven't read yet, it's not ideal
            // but we drop them here.
            // In future this should be changed to become more reliable.
            let mut buf = Vec::new();

            loop {
                self.stream.next_full_message(&mut buf, MAX_MSG_LEN).await?;

                if buf.starts_with(b"FILEDATA_") {
                    break;
                }
            }

            Result::<_, TakeFileManagerError>::Ok(buf)
        })
        .await
        .map_err(|_| TakeFileManagerError::Timeout)??;

        let prefix_len = b"FILEDATA_".len();
        buf.copy_within(prefix_len.., 0);
        buf.truncate(buf.len() - prefix_len);

        Ok(buf)
    }

    /// Get access to the underlying stream
    /// TODO: in future have an abstraction around this and
    /// make stream pub(crate)
    pub fn stream_mut(&mut self) -> &mut Stream<UnixStream> {
        &mut self.stream
    }

    pub async fn shutdown(self) -> Result<(), ShutdownError> {
        self.vm.shutdown().await?;
        let tempdir = self.tempdir;
        tokio::task::spawn_blocking(move || tempdir.close()).await??;
        Ok(())
    }
}
