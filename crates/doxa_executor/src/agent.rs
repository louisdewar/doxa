use std::time::Duration;

use doxa_core::{
    actix_web::http::header::{ContentDisposition, CONTENT_DISPOSITION},
    error::StatusCode,
    tokio,
    tracing::info,
};
use doxa_vm::{
    error::{AgentLifecycleManagerError, ShutdownError, TakeFileManagerError},
    stream::MessageReader,
    Manager as VM,
};
use tokio::time::timeout;

use crate::{
    error::{AgentError, AgentShutdown, NextEventError, NextMessageError, Timeout},
    Settings,
};

pub const MAX_MSG_LEN: usize = 5_000;

pub struct VMAgent {
    id: String,
    vm_manager: VM,
    message_reader: MessageReader,
    /// Whether the process has finished or not
    finished: bool,
}

pub enum AgentEvent<'a> {
    Finished,
    Line(&'a [u8]),
}

impl VMAgent {
    pub async fn new(
        competition: &str,
        agent_ram_mb: usize,
        agent_id: String,
        storage: &doxa_storage::AgentRetrieval,
        settings: &Settings,
    ) -> Result<VMAgent, AgentError> {
        let agent_response = storage.download_agent(&agent_id, competition).await?;

        if agent_response.status() == StatusCode::GONE {
            return Err(AgentError::AgentGone);
        }

        if agent_response.status() == StatusCode::NOT_FOUND {
            return Err(AgentError::AgentNotFound);
        }

        if agent_response.status() != StatusCode::OK {
            return Err(AgentError::BadStatusCode);
        }

        let content_disposition = agent_response
            .headers()
            .get(CONTENT_DISPOSITION)
            .ok_or(AgentError::CouldNotExtractFilename)?;

        let content_disposition = ContentDisposition::from_raw(content_disposition)
            .map_err(|_| AgentError::CouldNotExtractFilename)?;

        let agent_name = content_disposition
            .get_filename()
            .ok_or(AgentError::CouldNotExtractFilename)?;

        let agent_size = agent_response
            .content_length()
            .ok_or(AgentError::CouldNotExtractFileSize)?;

        let mut vm = VM::new(
            settings.rootfs.clone(),
            settings.kernel_img.clone(),
            settings.kernel_boot_args.clone(),
            settings.firecracker_path.clone(),
            agent_ram_mb,
        )
        .await?;

        timeout(
            Duration::from_secs(60),
            vm.send_agent(agent_name, agent_size, agent_response.bytes_stream()),
        )
        .await
        .map_err(|_| Timeout {
            during: "send_agent".to_string(),
        })??;

        let agent = VMAgent {
            vm_manager: vm,
            id: agent_id,
            message_reader: MessageReader::new(Vec::new(), MAX_MSG_LEN),
            finished: false,
        };

        Ok(agent)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    /// See [`doxa_vm::Manager::send_agent_input`]
    pub async fn send_agent_input(&mut self, msg: &[u8]) -> Result<(), std::io::Error> {
        self.vm_manager.send_agent_input(msg).await
    }

    /// See [`doxa_vm::Manager::reboot_agent`]
    pub async fn reboot(&mut self, args: Vec<String>) -> Result<(), AgentLifecycleManagerError> {
        self.vm_manager.reboot_agent(args).await
    }

    /// See [`doxa_vm::Manager::reboot_agent`]
    pub async fn take_file(&mut self, path: String) -> Result<Vec<u8>, TakeFileManagerError> {
        self.vm_manager.take_file(path).await
    }

    /// Retrieves the next event sent by the VMExecutor.
    /// This method is cancel safe.
    async fn next_event(&mut self) -> Result<AgentEvent<'_>, NextEventError> {
        let msg = self
            .message_reader
            .read_full_message(&mut self.vm_manager.stream_mut())
            .await?;

        let split_location = msg
            .iter()
            .position(|b| *b == b'_')
            .ok_or(NextEventError::MissingSeparator)?;
        let (prefix, msg) = msg.split_at(split_location);
        // Exclude the _ character itself
        let msg = &msg[1..];

        match prefix {
            b"OUTPUT" => {
                // This is currently a line of output (without the newline)
                return Ok(AgentEvent::Line(msg));
            }
            b"F" => {
                self.finished = true;
                info!(stderr = %String::from_utf8_lossy(msg), agent_id = %self.id, "agent stderr output");
                Ok(AgentEvent::Finished)
            }
            _ => Err(NextEventError::UnrecognisedPrefix),
        }
    }

    /// Retrieves the next message (full line) emitted by the agent inside the VM.
    /// This method is cancel safe.
    pub async fn next_message(&mut self) -> Result<&[u8], NextMessageError> {
        if self.finished {
            return Err(NextMessageError::Shutdown(AgentShutdown));
        }

        match self.next_event().await? {
            AgentEvent::Line(msg) => Ok(msg),
            AgentEvent::Finished => Err(NextMessageError::Shutdown(AgentShutdown)),
        }
    }

    pub async fn shutdown(self) -> Result<(), ShutdownError> {
        self.vm_manager.shutdown().await
    }
}
