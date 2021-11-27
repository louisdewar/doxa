use std::ffi::OsStr;
use std::path::Path;

use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio::process::{Child, ChildStdin, ChildStdout};

use crate::executor::spawn::Python;
use crate::{error::ExecutionSpawnError, executor::spawn, ExecutionConfig};

/// Represents a running agent
pub struct RunningAgent {
    pub child_process: Child,
    pub stdin: ChildStdin,
    stdout_lines: Lines<BufReader<ChildStdout>>,
}

impl RunningAgent {
    /// The root is the directory of the config.
    pub async fn spawn(
        config: &ExecutionConfig,
        root: &Path,
        args: Vec<&OsStr>,
    ) -> Result<RunningAgent, ExecutionSpawnError> {
        // TODO: canonicalise entrypoint and check it's below /home/doxa

        let mut child_process = match &config.language {
            &crate::Language::Python => {
                spawn::spawn::<Python>(root, &config.options, &config.entrypoint, args)
            }
            lang => todo!("language {:?}", lang),
        }?;

        let stdout = child_process.stdout.take().unwrap();
        let stdin = child_process.stdin.take().unwrap();

        Ok(RunningAgent {
            child_process,
            stdin,
            stdout_lines: BufReader::new(stdout).lines(),
        })
    }

    pub async fn next_line(&mut self) -> std::io::Result<Option<String>> {
        self.stdout_lines.next_line().await
    }
}
