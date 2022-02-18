use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use serde::Serialize;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};

use crate::error::{ShutdownError, SpawnError};

#[derive(Serialize)]
pub struct MachineConfig {
    pub vcpu_count: u8,
    pub mem_size_mib: u64,
}

#[derive(Serialize)]

pub struct Vsock {
    pub vsock_id: String,
    pub guest_cid: u8,
    pub uds_path: String,
}

#[derive(Serialize)]
pub struct BootSource {
    pub kernel_image_path: String,
    pub boot_args: String,
}

#[derive(Serialize)]
pub struct DriveSource {
    pub drive_id: String,
    pub path_on_host: String,
    pub is_root_device: bool,
    pub is_read_only: bool,
}

#[derive(Serialize)]
pub struct VMOptions {
    #[serde(rename = "boot-source")]
    pub boot_source: BootSource,
    pub drives: Vec<DriveSource>,
    #[serde(rename = "machine-config")]
    pub machine_config: MachineConfig,
    pub vsock: Vsock,
}

impl VMOptions {
    pub async fn save(&self, output_path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let mut f = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(output_path)
            .await?;

        f.write_all(serde_json::to_string(&self).unwrap().as_bytes())
            .await?;

        Ok(())
    }
}

/// A running VM
pub struct VM {
    firecracker_process: Child,
}

impl VM {
    /// This will spawn the firecracker process and then wait for it to create the socket file
    /// (unless it reaches a MAX timeout).
    /// Then it will send the initial settings such as memory size, kernel image etc..
    ///
    /// If there is an error after the process has been created then this will try to terminate the
    /// process as best as it can although it may fail in which case it will panic.
    pub async fn spawn(
        options_path: impl AsRef<OsStr>,
        firecracker_path: PathBuf,
    ) -> Result<VM, SpawnError> {
        let firecracker_process = Command::new(firecracker_path)
            .arg("--config-file")
            .arg(options_path)
            .arg("--no-api")
            // .arg("--api-sock")
            // .arg(&self.socket)
            .stdin(Stdio::null())
            // If this isn't consumed then the VM can run out of memory
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let vm = VM {
            firecracker_process,
        };

        Ok(vm)
    }

    pub fn firecracker_process(&mut self) -> &mut Child {
        &mut self.firecracker_process
    }

    /// Attempts a best effort to tidy up VM resources including terminating the process
    pub async fn shutdown(mut self) -> Result<(), ShutdownError> {
        self.firecracker_process.kill().await?;
        // tokio::task::spawn_blocking(move || self.socket.close()).await??;
        Ok(())
    }
}
