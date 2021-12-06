use std::{io, path::PathBuf};

use doxa_core::tokio;
use rand::Rng;
use tokio::fs::{File, OpenOptions};

/// 8 would mean a u64 equivalent of randomness which should be plenty.
const FILE_NAME_LENGTH: usize = 10;

pub struct LocalStorage {
    root: PathBuf,
}

impl LocalStorage {
    pub fn new(root: PathBuf) -> Self {
        LocalStorage { root }
    }

    /// Generates a new random name for a file with a fixed constant length of bytes that are then
    /// converted to hexidecimal.
    pub fn generate_name() -> String {
        rand::thread_rng()
            .sample_iter(rand::distributions::Standard)
            .take(FILE_NAME_LENGTH)
            .map(|b: u8| format!("{:02x}", b))
            .collect()
    }

    /// Generates a random name for the file creating it and then returning both the file and the
    /// name.
    ///
    /// The file is opened with write access and it is required that the file was created (so no
    /// file existed there before.
    pub async fn create_file(&self, competition_name: String) -> io::Result<(File, String)> {
        let folder = self.root.join(competition_name);

        tokio::fs::create_dir_all(&folder).await?;

        let file_name = Self::generate_name();
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(folder.join(&file_name))
            .await
            .map(|f| (f, file_name))
    }

    pub async fn open_file(&self, competition_name: &str, agent_id: &str) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .open(self.root.join(&competition_name).join(&agent_id))
            .await
    }
}
