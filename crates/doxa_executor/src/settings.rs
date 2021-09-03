use std::path::PathBuf;

use doxa_storage::AgentRetrieval;

pub struct Settings {
    pub firecracker_path: PathBuf,
    pub kernel_img: PathBuf,
    pub kernel_boot_args: String,
    pub rootfs: PathBuf,
    pub agent_retrieval: AgentRetrieval,
}
