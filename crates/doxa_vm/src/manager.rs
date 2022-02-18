use crate::error::{
    AgentLifecycleManagerError, ManagerErrorLogContext, MountError, TakeFileManagerError,
    VMShutdownError,
};
use crate::manager::ManagerError::TimeoutWaitingForVMConnection;
use crate::mount::{self, Mount, MountRequest};
use crate::recorder::VMRecorder;
use doxa_firecracker_sdk::spawn::{BootSource, DriveSource, MachineConfig};
use std::time::Duration;
use std::{io, path::PathBuf};
use tokio::time::timeout;

use doxa_firecracker_sdk::{VMOptions, VM};
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

const MAX_LOGS_LEN: usize = 50_000_000;

/// Manages lifecycle of and communciation with a single VM.
pub struct Manager {
    vm: VM,
    tempdir: TempDir,
    stream: Stream<UnixStream>,
    recorder: VMRecorder,
}

pub struct VMManagerArgs {
    pub original_rootfs: PathBuf,
    pub kernel_img: PathBuf,
    pub kernel_boot_args: String,
    pub firecracker_path: PathBuf,
    pub memory_size_mib: u64,
    pub swap_size_mib: u64,
    pub scratch_source_path: PathBuf,
    pub scratch_size_mib: u64,
    pub mounts: Vec<Mount>,
}

impl Manager {
    /// Spawns up the firecracker process and waits for the process to connect.
    /// This will copy the rootfs to a tempdir so that the VM is allowed write permissions.
    /// In future it will be a good idea to figure out properly how to make a rootfs that can be
    /// booted with read_only access into RAM.
    ///
    /// This will also create a scratch file system with a specific size to be mounted at /scratch
    /// Additional file systems to mount can also be specified in the vec.
    /// The scratch source path is used as a base for the scratch image, it is not modified in any
    /// way.
    /// The path on guest is a String since it's serialized as such before sending across.
    pub async fn new(args: VMManagerArgs) -> Result<Self, ManagerErrorLogContext> {
        let VMManagerArgs {
            original_rootfs,
            kernel_img,
            kernel_boot_args,
            firecracker_path,
            memory_size_mib,
            swap_size_mib,
            scratch_source_path,
            scratch_size_mib,
            mut mounts,
        } = args;

        // TODO: consider that when Drop is called for the tempdir by default it will be blocking,
        // maybe implement a custom Drop for manager that calls spawn_blocking?
        // Also if the executor is typically run as it's own process in future it may not matter,
        // it will probably only be a problem if it's being run on the main webserver process.
        let dir = task::spawn_blocking(tempdir).await??;

        let rootfs_path = dir.path().join("rootfs");
        tokio::fs::copy(original_rootfs, &rootfs_path).await?;

        let scratch_path = dir.path().join("scratch");
        mount::create_scratch_on_host(scratch_source_path, &scratch_path, scratch_size_mib)
            .await
            .map_err(ManagerError::CreateScratch)?;

        let swap_path = dir.path().join("swap");
        mount::create_swapfile_on_host(&swap_path, swap_size_mib)
            .await
            .map_err(ManagerError::CreateSwap)?;

        mounts.push(Mount {
            path_on_host: scratch_path,
            path_on_guest: "/scratch".to_string(),
            read_only: false,
        });

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
                kernel_image_path: kernel_img.to_string_lossy().to_string(),
                boot_args: kernel_boot_args,
            },
            drives: drive_sources,
            machine_config: MachineConfig {
                vcpu_count: 4,
                mem_size_mib: memory_size_mib,
            },
            vsock: doxa_firecracker_sdk::spawn::Vsock {
                vsock_id: "1".into(),
                guest_cid: 3,
                uds_path: dir.path().join("v.sock").to_string_lossy().to_string(),
            },
        };

        let vm_options_path = dir.path().join("vm_options.json");
        vm_options.save(&vm_options_path).await?;

        // Begin listening for connections on port 1001 before VM boots up
        let listener = UnixListener::bind(dir.path().join("v.sock_1001")).unwrap();
        let mut vm = VM::spawn(vm_options_path, firecracker_path).await?;

        let stdout = vm.firecracker_process().stdout.take().unwrap();
        let stderr = vm.firecracker_process().stderr.take().unwrap();

        let recorder = VMRecorder::start(stdout, stderr, MAX_LOGS_LEN);

        // We wrap this section because if there's an error we probably want the VM logs for
        // debugging and it's quite a hassle to write that code for each error.
        let stream = match Self::startup_process(listener, &mount_request).await {
            Ok(stream) => stream,
            Err(e) => {
                let logs = recorder.retrieve_logs().await;

                return Err(ManagerErrorLogContext {
                    source: e,
                    logs: Some(logs),
                });
            }
        };

        let manager = Manager {
            vm,
            tempdir: dir,
            stream,
            recorder,
        };
        Ok(manager)
    }

    async fn startup_process(
        listener: UnixListener,
        mount_request: &MountRequest,
    ) -> Result<Stream<UnixStream>, ManagerError> {
        let (stream, _addr) = timeout(Duration::from_secs(45), listener.accept())
            .await
            .map_err(|_| TimeoutWaitingForVMConnection)??;

        let mut stream = Stream::from_socket(stream);

        Self::mount_drives(&mut stream, mount_request).await?;

        Ok(stream)
    }

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

        stream.expect_exact_msg(b"MOUNTED").await?;

        Ok(())
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

    /// Shutdown the VM and retrieve the logs
    pub async fn shutdown(self) -> Result<String, VMShutdownError> {
        self.vm.shutdown().await?;
        let tempdir = self.tempdir;
        tokio::task::spawn_blocking(move || tempdir.close()).await??;

        let logs = self.recorder.retrieve_logs().await?;
        Ok(logs)
    }
}
