use std::path::{Path, PathBuf};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    error::{ManagerError, VMShutdownError},
    mount::Mount,
    recorder::VMRecorder,
    stream::Stream,
};

pub mod docker;
pub mod firecracker;

#[async_trait::async_trait]
pub trait VMBackend: Sized + Send + Sync {
    type BackendSettings: Clone + Send;

    type Socket: AsyncRead + AsyncWrite + Unpin + Send + Sync;

    const SUPPORTS_MOUNTING_IMAGES: bool;

    async fn spawn(
        tempdir: &Path,
        backend_settings: Self::BackendSettings,
        ram_mib: u64,
        mounts: Vec<Mount>,
        swap_path: PathBuf,
    ) -> Result<Self, ManagerError>;

    fn take_recorder(&mut self, max_len: usize) -> VMRecorder;

    /// Connects to the VM manager inside the VM.
    async fn connect(&mut self) -> Result<Stream<Self::Socket>, ManagerError>;

    async fn shutdown(self) -> Result<(), VMShutdownError>;
}

