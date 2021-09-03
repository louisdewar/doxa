use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use tokio::process::{Child, Command};

use crate::{error::ExecutionSpawnError, Options};

use super::{DOXA_GID, DOXA_UID};

const PYTHON_BIN: &'static str = "/usr/bin/python";

pub async fn spawn_python(
    root: &Path,
    options: &Options,
    entrypoint: &str,
) -> Result<Child, ExecutionSpawnError> {
    Ok(Command::new(PYTHON_BIN)
        .arg(entrypoint)
        .current_dir(root)
        .uid(DOXA_UID)
        .gid(DOXA_GID)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?)
}
