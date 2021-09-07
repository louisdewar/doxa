use std::{io, path::PathBuf};

use doxa_firecracker_sdk::{error::ShutdownError, VMOptions, VM};
use tokio::{
    net::{UnixListener, UnixStream},
    task,
};

use tracing::{info, trace};

use crate::{
    error::{ManagerError, SendAgentError},
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
    ) -> Result<Self, ManagerError> {
        // TODO: consider that when Drop is called for the tempdir by default it will be blocking,
        // maybe implement a custom Drop for manager that calls spawn_blocking?
        // Also if the executor is typically run as it's own process in future it may not matter,
        // it will probably only be a problem if it's being run on the main webserver process.
        let dir = task::spawn_blocking(|| tempdir()).await??;

        let rootfs_path = dir.path().join("rootfs");

        tokio::fs::copy(original_rootfs, &rootfs_path).await?;

        let vm = VMOptions {
            memory_size_mib: 128,
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

        // TODO: timeout
        let (stream, _addr) = listener.accept().await?;

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
            .map_err(|e| SendAgentError::DownloadAgentError(e))?;

        self.stream
            .send_full_message("FILE ENDS".as_bytes())
            .await?;

        self.stream.expect_exact_msg(b"RECEIVED").await?;
        trace!("VM has received agent");

        self.stream.expect_exact_msg(b"SPAWNED").await?;
        info!("VM has spawned agent");

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

    /// Get access to the underlying stream
    /// TODO: in future have an abstraction around this and
    /// make stream pub(crate)
    pub fn stream_mut(&mut self) -> &mut Stream<UnixStream> {
        &mut self.stream
    }

    //pub async fn shutdown(self) -> Result<(ChildStdout, ChildStderr), ShutdownError> {
    pub async fn shutdown(self) -> Result<(), ShutdownError> {
        self.vm.shutdown().await?;
        let tempdir = self.tempdir;
        tokio::task::spawn_blocking(move || tempdir.close()).await??;
        //Ok((self.stdout, self.stderr))
        Ok(())
    }
}
