//! Everything relating to drives used by DOXA and mounting them in VMs.

pub(crate) const SWAP_UUID: &str = "9e5cfaba-005d-4f79-9391-4f1df84bfd4f";

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Stdio,
};

use serde::{Deserialize, Serialize};

use crate::error::RunCommandError;

#[derive(Clone)]
pub struct Mount {
    pub path_on_host: PathBuf,
    pub path_on_guest: String,
    pub read_only: bool,
}

#[derive(Serialize, Deserialize)]
/// Represents a request from the host to mount drives with specific UUID at paths.
pub struct MountRequest {
    /// Vec of tuples of (UUID, Path on guest, is_read_only)
    pub mounts: Vec<(String, String, bool)>,
}

impl Mount {}

/// Creates a scratch file at the destination location with a specified size in MB.
/// It requires the base scratch file to copy and extend.
pub async fn create_scratch_on_host(
    scratch_src: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    size_mb: u64,
) -> Result<(), RunCommandError> {
    tokio::fs::copy(scratch_src, &destination).await?;

    let status = tokio::process::Command::new("resize2fs")
        .arg(destination.as_ref())
        .arg(format!("{}M", size_mb))
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()
        .await?;

    if status.success() {
        Ok(())
    } else {
        Err(RunCommandError::BadExitCode(status))
    }
}

pub async fn create_swapfile_on_host(
    destination: impl AsRef<Path>,
    size_mb: u64,
) -> Result<(), RunCommandError> {
    let status = tokio::process::Command::new("dd")
        .arg("if=/dev/zero")
        .arg("bs=1M")
        .arg(format!("count={}", size_mb))
        .arg(format!("of={}", destination.as_ref().to_string_lossy()))
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()
        .await?;

    if !status.success() {
        return Err(RunCommandError::BadExitCode(status));
    }

    let status = tokio::process::Command::new("mkswap")
        .arg("-U")
        .arg(SWAP_UUID)
        .arg(destination.as_ref())
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()
        .await?;

    if !status.success() {
        return Err(RunCommandError::BadExitCode(status));
    }

    Ok(())
}

pub async fn swapon() -> Result<(), RunCommandError> {
    let status = tokio::process::Command::new("/sbin/swapon")
        .arg("-U")
        .arg(SWAP_UUID)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()
        .await?;

    if status.success() {
        Ok(())
    } else {
        Err(RunCommandError::BadExitCode(status))
    }
}

fn process_blkid_line(line: &[u8]) -> Option<(&[u8], &[u8])> {
    // The line is in the format:
    // DRIVE_PATH: KEY="VALUE" KEY2="VALUE2"
    // One of those keys is UUID
    let colon_position = line.iter().position(|b| *b == b':')?;

    let (drive_path, line) = line.split_at(colon_position);

    // Skip the ": "
    let mut line = &line[2..];

    while !line.is_empty() {
        let equal_position = line.iter().position(|b| *b == b'=')?;

        let key = &line[0..equal_position];
        line = &line[equal_position + 1..];

        if line[0] != b'"' {
            return None;
        }

        line = &line[1..];

        let quote_end_position = line.iter().position(|b| *b == b'"')?;
        let value = &line[0..quote_end_position];

        line = &line[quote_end_position + 1..];

        if key == b"UUID" {
            let uuid_value = value;

            return Some((drive_path, uuid_value));
        }
    }

    None
}

// Returns the UUID the given image path.
pub async fn get_image_uuid(image: impl AsRef<Path>) -> Result<String, RunCommandError> {
    let output = tokio::process::Command::new("/sbin/blkid")
        .arg(image.as_ref())
        .stdout(Stdio::piped())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait_with_output()
        .await?;

    Ok(String::from_utf8_lossy(
        process_blkid_line(&output.stdout)
            .expect("Failed to get image UUID")
            .1,
    )
    .to_string())
}

// Returns a hashmap of UUID string to path to drive (e.g. /dev/vdb)
pub async fn find_drives_by_uuid() -> Result<HashMap<String, String>, RunCommandError> {
    let output = tokio::process::Command::new("/sbin/blkid")
        .stdout(Stdio::piped())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait_with_output()
        .await?;

    if !output.status.success() {
        return Err(RunCommandError::BadExitCode(output.status));
    }

    let mut map = HashMap::new();

    for line in output.stdout.split(|b| *b == b'\n') {
        if let Some((drive_path, uuid)) = process_blkid_line(line) {
            map.insert(
                String::from_utf8_lossy(uuid).to_string(),
                String::from_utf8_lossy(drive_path).to_string(),
            );
        }
    }

    Ok(map)
}
