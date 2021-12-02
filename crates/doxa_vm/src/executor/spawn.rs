use std::{ffi::OsStr, path::Path, process::Stdio};

use tokio::process::{Child, Command};

use crate::{error::ExecutionSpawnError, Options};

use super::{DOXA_GID, DOXA_UID};

const PYTHON_BIN: &str = "/usr/bin/python";

// TODO: make this a bit more composable / abstracted, i.e. the parts about current_dir, uid, gid
// etc... should be abstracted

pub trait Spawnable {
    fn initiate(root: &Path, options: &Options, entrypoint: &str) -> Command;
}

pub fn spawn<T: Spawnable>(
    root: &Path,
    options: &Options,
    entrypoint: &str,
    args: Vec<&OsStr>,
) -> Result<Child, ExecutionSpawnError> {
    Ok(T::initiate(root, options, entrypoint)
        .args(args)
        .current_dir(root)
        .uid(DOXA_UID)
        .gid(DOXA_GID)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?)
}

pub struct Python;

impl Spawnable for Python {
    fn initiate(_root: &Path, _options: &Options, entrypoint: &str) -> Command {
        let mut command = Command::new(PYTHON_BIN);
        command.arg(entrypoint);

        command
    }
}
