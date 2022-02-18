use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use doxa_competition::{tokio, tracing::info};

use crate::error::{DatasetLoadingError, UnknownDataset};

pub struct Datasets {
    inner: HashMap<String, PhaseDataset>,
}

impl Datasets {
    pub async fn load_from_directory(
        datasets_directory: impl AsRef<Path>,
    ) -> Result<Self, DatasetLoadingError> {
        let mut read_dir = tokio::fs::read_dir(&datasets_directory)
            .await
            .map_err(DatasetLoadingError::ReadDir)?;

        let mut datasets = HashMap::new();

        while let Some(entry) = read_dir
            .next_entry()
            .await
            .map_err(DatasetLoadingError::ReadDir)?
        {
            let dataset_name = entry
                .file_name()
                .to_str()
                .expect("dataset name was not utf-8")
                .to_string();
            info!(%dataset_name, "discovered climatehack dataset");
            let dataset_path = entry.path();
            let x_train = dataset_path.join("x-train.img");

            let x_train_meta = tokio::fs::metadata(&x_train)
                .await
                .map_err(DatasetLoadingError::DatasetX)?;
            if !x_train_meta.is_file() {
                return Err(DatasetLoadingError::DatasetXNotFile);
            }

            let y_train = dataset_path.join("testset-y");
            let y_train_meta = tokio::fs::metadata(&y_train)
                .await
                .map_err(DatasetLoadingError::DatasetY)?;
            if !y_train_meta.is_dir() {
                return Err(DatasetLoadingError::DatasetYNotDirectory);
            }

            let dataset = PhaseDataset::new(y_train, x_train).await;
            assert!(datasets.insert(dataset_name, dataset).is_none());
        }

        Ok(Datasets { inner: datasets })
    }

    pub fn get_dataset(&self, name: &str) -> Result<&PhaseDataset, UnknownDataset> {
        self.inner.get(name).ok_or(UnknownDataset {
            name: name.to_string(),
        })
    }
}

#[derive(Clone)]
pub struct PhaseDataset {
    pub true_y_path: PathBuf,
    pub x_image_path: PathBuf,
    pub group_count: u32,
}

impl PhaseDataset {
    pub async fn new(true_y_path: PathBuf, x_image_path: PathBuf) -> PhaseDataset {
        let mut entries = tokio::fs::read_dir(&true_y_path)
            .await
            .expect("failed to read true y path");

        let mut count = 0;
        while let Some(entry) = entries
            .next_entry()
            .await
            .expect("failed to open dir entry")
        {
            if entry.file_name().to_string_lossy().ends_with(".npz") {
                count += 1;
            }
        }

        PhaseDataset {
            true_y_path,
            x_image_path,
            group_count: count,
        }
    }
}
