use std::io::ErrorKind;
use std::path::PathBuf;

use hyper::Method;
use tokio::fs::File;
use tokio::process::{Child, Command};

use tempfile::NamedTempFile;

use crate::error::{InvalidPath, RequestError, ShutdownError, SpawnError, TimeoutWaitingForSocket};
use crate::net::expect_ok_response;
use crate::request::{Action, BootSource, CreateVsock, MountDrive};

pub struct VMOptions {
    pub memory_size_mib: usize,
    pub vcpus: usize,
    pub kernel_image_path: PathBuf,
    pub kernel_boot_args: String,
    pub rootfs_path: PathBuf,
    pub rootfs_read_only: bool,
    pub socket: PathBuf,
}

/// A running VM
pub struct VM {
    firecracker_process: Child,
    socket: PathBuf,
}

impl VMOptions {
    /// This will spawn the firecracker process and then wait for it to create the socket file
    /// (unless it reaches a MAX timeout).
    /// Then it will send the initial settings such as memory size, kernel image etc..
    ///
    /// If there is an error after the process has been created then this will try to terminate the
    /// process as best as it can although it may fail in which case it will panic.
    pub async fn spawn(self, firecracker_path: PathBuf) -> Result<VM, SpawnError> {
        // let socket = tokio::task::spawn_blocking(|| NamedTempFile::new()).await??;

        let firecracker_process = Command::new(firecracker_path)
            .arg("--api-sock")
            .arg(&self.socket)
            .kill_on_drop(true)
            .spawn()?;

        let vm = VM {
            firecracker_process,
            socket: self.socket.clone(),
        };

        /// Any error that occurs during here needs to first cause the VM to be shut down
        async fn spawn_inner(options: VMOptions, vm: &VM) -> Result<(), SpawnError> {
            vm.wait_for_socket(5).await?;

            vm.set_kernel_image(
                options
                    .kernel_image_path
                    .to_str()
                    .ok_or(InvalidPath)?
                    .to_string(),
                options.kernel_boot_args,
            )
            .await?;

            vm.set_rootfs(
                options.rootfs_path.to_str().ok_or(InvalidPath)?.to_string(),
                options.rootfs_read_only,
            )
            .await?;

            Ok(())
        }

        match spawn_inner(self, &vm).await {
            Ok(()) => Ok(vm),
            Err(e) => {
                vm.shutdown()
                    .await
                    .expect("Could not shut down VM after there was an error spawning it");
                Err(e)
            }
        }
    }
}

impl VM {
    pub fn firecracker_process(&mut self) -> &mut Child {
        &mut self.firecracker_process
    }

    /// Makes n attempts at 1 second intervals for the socket to be created, if it is not created
    /// in that time an error is returned (TimeoutWaitingForSocket).
    /// If there is another other IO error then that is returned.
    /// The first attempt is made without delay.
    async fn wait_for_socket(&self, attempts: usize) -> Result<(), SpawnError> {
        for _ in 0..attempts {
            match File::open(&self.socket).await {
                Err(e) if e.kind() == ErrorKind::NotFound => {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
                // The socket isn't a regular file and can't be opened so when it's created
                // File::open will return an error but of a different kind
                _ => return Ok(()),
            }
        }

        Err(TimeoutWaitingForSocket.into())
    }

    async fn set_kernel_image(
        &self,
        kernel_image_path: String,
        boot_args: String,
    ) -> Result<(), RequestError> {
        let body = serde_json::to_string(&BootSource {
            kernel_image_path,
            boot_args,
        })
        .unwrap()
        .into();

        expect_ok_response(
            &crate::net::send_socket_request(&self.socket, Method::PUT, body, "/boot-source")
                .await?,
        )?;

        Ok(())
    }

    async fn set_rootfs(
        &self,
        rootfs_path: String,
        is_read_only: bool,
    ) -> Result<(), RequestError> {
        let body = serde_json::to_string(&MountDrive {
            drive_id: "rootfs".to_string(),
            path_on_host: rootfs_path,
            is_root_device: true,
            is_read_only,
        })
        .unwrap()
        .into();

        expect_ok_response(
            &crate::net::send_socket_request(&self.socket, Method::PUT, body, "/drives/rootfs")
                .await?,
        )?;

        Ok(())
    }

    /// This method can only be called before the VM has started.
    pub async fn create_vsock(
        &self,
        vsock_id: String,
        guest_cid: u32,
        uds_path: String,
    ) -> Result<(), RequestError> {
        let body = serde_json::to_string(&CreateVsock {
            vsock_id,
            guest_cid,
            uds_path,
        })
        .unwrap()
        .into();

        expect_ok_response(
            &crate::net::send_socket_request(&self.socket, Method::PUT, body, "/vsock").await?,
        )?;

        Ok(())
    }

    /// The method can only be successfully called once.
    pub async fn instance_start(&self) -> Result<(), RequestError> {
        let body = serde_json::to_string(&Action {
            action_type: "InstanceStart".to_string(),
        })
        .unwrap()
        .into();

        expect_ok_response(
            &crate::net::send_socket_request(&self.socket, Method::PUT, body, "/actions").await?,
        )?;

        Ok(())
    }

    /// Attempts a best effort to tidy up VM resources including terminating the process
    pub async fn shutdown(mut self) -> Result<(), ShutdownError> {
        self.firecracker_process.kill().await?;
        // tokio::task::spawn_blocking(move || self.socket.close()).await??;
        Ok(())
    }
}
