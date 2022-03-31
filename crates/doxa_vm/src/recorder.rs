use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::sync::oneshot;
use tokio::task;
use tokio::time::timeout;
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

use crate::error::VMRecorderError;

const INIT_STARTED_MESSAGE: &str = "DOXA INIT started";

/// A way of recording the output of the VM for logging purposes.
/// Separates Stdout and Stderr.
/// If the VM gets through bootup to the point where it starts the executor inside the VM, the logs
/// will truncate the output during bootup.
/// Note: this will not record the output of the agent.
pub struct VMRecorder {
    handle: task::JoinHandle<Result<String, std::io::Error>>,
    shutdown: oneshot::Sender<()>,
}

impl VMRecorder {
    /// Starts the task asynchronously reading logs.
    /// If max_len is reached during then no more data is read into the logs (effective the first n
    /// bytes of the true logs).
    /// If this occurs during bootup then the logs will still be reset after the VM executor
    /// starts (and the len reset to 0).
    ///
    /// NOTE: max_len should be considered a soft limit.
    pub fn start(
        stdout: impl AsyncRead + Unpin + Send + 'static,
        stderr: impl AsyncRead + Unpin + Send + 'static,
        max_len: usize,
    ) -> Self {
        let (sender, receiver) = oneshot::channel();

        let handle = tokio::spawn(async move {
            let mut shutdown = receiver;
            let mut logs = String::new();
            let stdout = LinesStream::new(BufReader::new(stdout).lines());
            let stderr = LinesStream::new(BufReader::new(stderr).lines());
            let mut lines = StreamExt::merge(stdout, stderr);

            let mut bootup = true;

            loop {
                tokio::select! {
                    Some(line) = lines.next() => {
                        let line = line?;
                        if bootup && line.contains(INIT_STARTED_MESSAGE) {
                            bootup = false;
                            logs.truncate(0);
                        }

                        if logs.len() + line.len() >= max_len {
                            continue;
                        }


                        // TODO: in future use escape_ascii
                        // Perform escaping within the line (but don't escape the newline)
                        let line = line.replace('\u{0}', "{NULL_BYTE}");

                        logs.push_str(&line);
                        logs.push('\n');
                    },
                    // Either shutdown yields Ok(()) (when the sender is manually called) or
                    // Err(_) (when the VMRecorder struct gets dropped without specifically asking
                    // for the logs), in either case we just return with the logs we have so far
                    _ = &mut shutdown => {
                        logs.push_str("Logs stopping due to signal");
                        break
                    },
                    // The VM has shutdown so we yield the logs
                    else => {
                        logs.push_str("Logs stopping due to VM shutting down");
                        break
                    },
                }
            }

            Ok(logs)
        });

        VMRecorder {
            handle,
            shutdown: sender,
        }
    }

    // async fn retrieve_logs_passive(&mut self) -> Result<String, VMRecorderError> {
    //     let logs = (&mut self.handle).await??;

    //     Ok(logs)
    // }

    async fn retrieve_logs_active(self) -> Result<String, VMRecorderError> {
        // Wait a few seconds to try to let the recorder exit by itself and get all the log lines
        tokio::time::sleep(Duration::from_secs(3)).await;

        // If this fails it means that the other end has already terminated so we can just get the
        // result.
        let _ = self.shutdown.send(());
        let logs = timeout(Duration::from_secs(5), self.handle)
            .await
            .map_err(|_| VMRecorderError::Timeout)???;

        Ok(logs)
    }

    /// Retrieves the logs from the VM.
    ///
    /// **these steps (marked with x) are currently skipped because I'm not sure about the
    /// guarantees of cancelling a JoinHandle future, only the active shutdown happens (with
    /// timeout)**
    /// x This first tries to see if the logs are ready without any intervention.
    /// x There is a timeout to allow the recorder to finish processing the logs assuming the VM has
    /// x exited it should quickly return the logs.
    ///
    /// If the timeout is reached then it's assumed that the recorder hasn't detected the VM
    /// shutdown, in which case this asks the recorder to stop reading logs and return what it
    /// currently has.
    /// If there is no response after a timeout then this returns with an error.
    pub async fn retrieve_logs(self) -> Result<String, VMRecorderError> {
        self.retrieve_logs_active().await
    }
}
