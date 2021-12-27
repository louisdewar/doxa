use std::path::PathBuf;

use doxa_storage::AgentRetrieval;
pub use doxa_vm::mount::Mount;

pub struct Settings {
    pub firecracker_path: PathBuf,
    pub kernel_img: PathBuf,
    pub kernel_boot_args: String,
    pub rootfs: PathBuf,
    pub agent_retrieval: AgentRetrieval,
    pub scratch_base_image: PathBuf,
    /// A list of file systems to mount for every competition (the scratch and rootfs are mounted
    /// automatically and do not need to be listed here).
    /// Competitions can also define additional mounts on top of these.
    pub base_mounts: Vec<Mount>,
}
