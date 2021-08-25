use std::path::PathBuf;

use doxa_firecracker_sdk::{error::ShutdownError, VMOptions, VM};
use tokio::{
    io::AsyncRead,
    net::{UnixListener, UnixStream},
    task,
};

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

        // let mut vm_stdout = vm.firecracker_process().stdout.take().unwrap();
        // task::spawn(async move {
        //     tokio::io::copy(&mut vm_stdout, &mut tokio::io::stdout())
        //         .await
        //         .unwrap();
        // });

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

        Ok(Manager {
            vm,
            tempdir: dir,
            stream,
        })
    }

    pub async fn send_agent<R: AsyncRead + Unpin>(
        &mut self,
        agent_name: String,
        agent_size: u64,
        mut agent_stream: R,
    ) -> Result<(), SendAgentError> {
        let mut length_message = [0; 9];
        length_message[0] = b'F';
        length_message[1..9].copy_from_slice(&u64::to_be_bytes(agent_size));
        self.stream.send_message(&length_message, false).await?;

        self.stream
            .send_message(format!("N{}", agent_name).as_bytes(), true)
            .await?;

        tokio::io::copy(&mut agent_stream, self.stream.get_writer()).await?;

        self.stream
            .send_message("FILE ENDS".as_bytes(), true)
            .await?;

        let mut msg = [0; 8];
        self.stream.read_exact(&mut msg).await?;

        if &msg != b"RECEIVED" {
            return Err(SendAgentError::MissingReceivedMessage);
        }

        Ok(())
    }

    pub async fn shutdown(self) -> Result<(), ShutdownError> {
        self.vm.shutdown().await?;
        let tempdir = self.tempdir;
        tokio::task::spawn_blocking(move || tempdir.close()).await??;
        Ok(())
    }
}
