use std::time::Duration;

use doxa_core::{
    actix_web::http::header::{ContentDisposition, CONTENT_DISPOSITION},
    error::StatusCode,
    tokio,
    tracing::warn,
};
use doxa_vm::{
    backend::VMBackend,
    error::{AgentLifecycleManagerError, TakeFileManagerError, VMShutdownError},
    manager::VMManagerSettings,
    mount::Mount,
    stream::{MessageLen, MessageReader},
    Manager as VM,
};
use tokio::time::timeout;

use crate::{
    error::{AgentError, AgentErrorLogContext, NextEventError, NextMessageError, Timeout},
    Settings,
};

pub const MAX_MSG_LEN: usize = 50_000;

pub struct VMAgent<B: VMBackend> {
    id: String,
    vm_manager: doxa_vm::manager::Manager<B>,
    message_reader: MessageReader,
    /// Whether the process is running or not
    running: bool,
}

pub enum AgentEvent<'a> {
    Finished { stderr: &'a [u8] },
    Line(&'a [u8]),
}

#[derive(Clone)]
pub struct VMAgentSettings {
    pub agent_ram_mb: u64,
    pub scratch_size_mb: u64,
    pub swap_size_mb: u64,
    /// All mounts excluding the scratch and rootfs which are mounted automatically
    pub mounts: Vec<Mount>,
}

impl<B: VMBackend> VMAgent<B> {
    pub async fn new(
        competition: &str,
        agent_id: String,
        storage: &doxa_storage::AgentRetrieval,
        settings: &Settings,
        vm_agent_settings: VMAgentSettings,
        backend_settings: B::BackendSettings,
    ) -> Result<VMAgent<B>, AgentErrorLogContext> {
        let agent_response = storage.download_agent(&agent_id, competition).await?;

        if agent_response.status() == StatusCode::GONE {
            return Err(AgentError::AgentGone.into());
        }

        if agent_response.status() == StatusCode::NOT_FOUND {
            return Err(AgentError::AgentNotFound.into());
        }

        if agent_response.status() != StatusCode::OK {
            warn!(status=%agent_response.status(), "bad status code");
            return Err(AgentError::BadStatusCode.into());
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

        // let vm_args = VMManagerArgs {
        //     original_rootfs: settings.rootfs.clone(),
        //     kernel_img: settings.kernel_img.clone(),
        //     kernel_boot_args: settings.kernel_boot_args.clone(),
        //     firecracker_path: settings.firecracker_path.clone(),
        //     memory_size_mib: vm_agent_settings.agent_ram_mb,
        //     scratch_source_path: settings.scratch_base_image.clone(),
        //     scratch_size_mib: vm_agent_settings.scratch_size_mb,
        //     swap_size_mib: vm_agent_settings.swap_size_mb,
        //     mounts: vm_agent_settings.mounts,
        // };

        let vm_settings = VMManagerSettings {
            swap_size_mib: vm_agent_settings.swap_size_mb,
            scratch_source_path: settings.scratch_base_image.clone(),
            scratch_size_mib: vm_agent_settings.scratch_size_mb,
            memory_size_mib: vm_agent_settings.agent_ram_mb,
            mounts: vm_agent_settings.mounts,
        };

        let mut vm = VM::new(vm_settings, backend_settings).await?;

        match async {
            timeout(
                Duration::from_secs(60 * 10),
                vm.send_agent(agent_name, agent_size, agent_response.bytes_stream()),
            )
            .await
            .map_err(|_| Timeout {
                during: "send_agent".to_string(),
            })??;

            Ok(())
        }
        .await
        {
            Ok(()) => {}
            Err(e) => {
                let logs = Some(vm.shutdown().await);

                return Err(AgentErrorLogContext { source: e, logs });
            }
        }

        let agent = VMAgent {
            vm_manager: vm,
            id: agent_id,
            message_reader: MessageReader::new(Vec::new(), MAX_MSG_LEN as MessageLen),
            running: false,
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
        self.vm_manager.reboot_agent(args).await?;
        self.running = true;
        Ok(())
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
            .read_full_message(self.vm_manager.stream_mut())
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
                self.running = false;
                Ok(AgentEvent::Finished { stderr: msg })
            }
            _ => Err(NextEventError::UnrecognisedPrefix),
        }
    }

    /// Retrieves the next message (full line) emitted by the agent inside the VM.
    /// This method is cancel safe.
    ///
    /// If the we just received the shutdown message then the error will contain the stderr from
    /// the agent.
    pub async fn next_message(&mut self) -> Result<&[u8], NextMessageError> {
        if !self.running {
            return Err(NextMessageError::Terminated { stderr: None });
        }

        match self.next_event().await? {
            AgentEvent::Line(msg) => Ok(msg),
            AgentEvent::Finished { stderr } => Err(NextMessageError::Terminated {
                stderr: Some(String::from_utf8_lossy(stderr).to_string()),
            }),
        }
    }

    pub async fn shutdown(self) -> Result<String, VMShutdownError> {
        self.vm_manager.shutdown().await
    }
}
