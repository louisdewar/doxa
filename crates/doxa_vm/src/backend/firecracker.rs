use std::{path::PathBuf, time::Duration};

use doxa_firecracker_sdk::{
    spawn::{BootSource, DriveSource, MachineConfig},
    VMOptions, VM,
};
use tokio::{
    net::{UnixListener, UnixStream},
    time::timeout,
};

use crate::{
    error::{ManagerError, MountError},
    mount::{self, MountRequest},
    recorder::VMRecorder,
    stream::Stream,
};

use super::VMBackend;

pub struct FirecrackerBackend {
    vm: VM,
    vsock_path: PathBuf,
    mount_request: MountRequest,
}

impl FirecrackerBackend {
    async fn mount_drives(
        stream: &mut Stream<UnixStream>,
        mount_request: &MountRequest,
    ) -> Result<(), MountError> {
        stream
            .send_prefixed_full_message(
                b"MOUNTREQUEST_",
                serde_json::to_vec(mount_request).unwrap().as_ref(),
            )
            .await?;

        stream.send_full_message(b"SWAPON").await?;

        stream.expect_exact_msg(b"MOUNTED").await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct FirecrackerBackendSettings {
    pub kernel_img: PathBuf,
    pub kernel_boot_args: String,
    pub firecracker_path: PathBuf,
    pub vcpus: u64,
    pub original_rootfs: std::path::PathBuf,
}

#[async_trait::async_trait]
impl VMBackend for FirecrackerBackend {
    type BackendSettings = FirecrackerBackendSettings;

    type Socket = UnixStream;

    const SUPPORTS_MOUNTING_IMAGES: bool = false;

    async fn spawn(
        tempdir: &std::path::Path,
        backend_settings: Self::BackendSettings,
        ram_mib: u64,
        mounts: Vec<crate::mount::Mount>,
        swap_path: std::path::PathBuf,
    ) -> Result<Self, crate::error::ManagerError> {
        let rootfs_path = tempdir.join("rootfs");
        tokio::fs::copy(backend_settings.original_rootfs, &rootfs_path).await?;

        let mut mount_request = MountRequest {
            mounts: Vec::with_capacity(mounts.len()),
        };

        let mut drive_sources = Vec::with_capacity(mounts.len() + 1);
        drive_sources.push(DriveSource {
            drive_id: "rootfs".into(),
            path_on_host: rootfs_path.to_string_lossy().to_string(),
            is_root_device: true,
            is_read_only: false,
        });

        drive_sources.push(DriveSource {
            drive_id: "swap".into(),
            path_on_host: swap_path.to_string_lossy().to_string(),
            is_root_device: false,
            is_read_only: false,
        });

        for (i, mount) in mounts.into_iter().enumerate() {
            drive_sources.push(DriveSource {
                drive_id: format!("drive_{}", i),
                path_on_host: mount.path_on_host.to_string_lossy().to_string(),
                is_root_device: false,
                is_read_only: mount.read_only,
            });

            let uuid = mount::get_image_uuid(mount.path_on_host)
                .await
                .map_err(ManagerError::GetImageUUID)?;

            mount_request
                .mounts
                .push((uuid, mount.path_on_guest, mount.read_only));
        }

        let vm_options = VMOptions {
            boot_source: BootSource {
                kernel_image_path: backend_settings.kernel_img.to_string_lossy().to_string(),
                boot_args: backend_settings.kernel_boot_args,
            },
            drives: drive_sources,
            machine_config: MachineConfig {
                vcpu_count: 4,
                mem_size_mib: ram_mib,
            },
            vsock: doxa_firecracker_sdk::spawn::Vsock {
                vsock_id: "1".into(),
                guest_cid: 3,
                uds_path: tempdir.join("v.sock").to_string_lossy().to_string(),
            },
        };

        let vm_options_path = tempdir.join("vm_options.json");
        vm_options.save(&vm_options_path).await?;

        let vm = VM::spawn(vm_options_path, backend_settings.firecracker_path).await?;

        let vsock_path = tempdir.join("v.sock_1001");

        Ok(FirecrackerBackend {
            vm,
            vsock_path,
            mount_request,
        })

        // let stdout = vm.firecracker_process().stdout.take().unwrap();
        // let stderr = vm.firecracker_process().stderr.take().unwrap();

        // let recorder = VMRecorder::start(stdout, stderr, MAX_LOGS_LEN);

        // // Begin listening for connections on port 1001
        // let listener = UnixListener::bind(dir.path().join("v.sock_1001")).unwrap();

        // // We wrap this section because if there's an error we probably want the VM logs for
        // // debugging and it's quite a hassle to write that code for each error.
        // let stream = match Self::startup_process(listener, &mount_request).await {
        //     Ok(stream) => stream,
        //     Err(e) => {
        //         let logs = recorder.retrieve_logs().await;

        //         return Err(ManagerErrorLogContext {
        //             source: e,
        //             logs: Some(logs),
        //         });
        //     }
        // };
    }

    fn take_recorder(&mut self, max_len: usize) -> crate::recorder::VMRecorder {
        let stdout = self.vm.firecracker_process().stdout.take().unwrap();
        let stderr = self.vm.firecracker_process().stderr.take().unwrap();

        VMRecorder::start(stdout, stderr, max_len)
    }

    async fn connect(&mut self) -> Result<Stream<Self::Socket>, crate::error::ManagerError> {
        let listener = UnixListener::bind(&self.vsock_path).unwrap();
        let (stream, _addr) = timeout(Duration::from_secs(45), listener.accept())
            .await
            .map_err(|_| crate::error::ManagerError::TimeoutWaitingForVMConnection)??;

        let mut stream = Stream::from_socket(stream);

        Self::mount_drives(&mut stream, &self.mount_request).await?;

        // Self::mount_drives(&mut stream, mount_request).await?;

        Ok(stream)
    }

    async fn shutdown(self) -> Result<(), crate::error::VMShutdownError> {
        self.vm.shutdown().await?;

        Ok(())
    }
}
