use std::{ffi::OsStr, path::PathBuf, process::Stdio};

use doxa_competition::{
    client::serde_json,
    tokio::{self, io::AsyncWriteExt},
};
use serde::Deserialize;

use crate::error::ScorerError;

// Due to incompatabilities with xarray and the zarr specification it was much simpler to simply
// run a python script with the xarray to do the scoring.
const SCORER_SCRIPT: &str = include_str!("./scorer.py");

/// This struct enables the game_client to save a python script to the work directory and then
/// subsequently call that script with paths to various agent outputs as input with the scorer
/// outputing the float value of the calculated metric.
pub struct Scorer {
    script_path: PathBuf,
    python_bin: PathBuf,
}

#[derive(Deserialize)]
struct Score {
    score: f64,
    #[serde(default)]
    error: Option<String>,
}

impl Scorer {
    /// Creates a new scorer object by outputting the python script to the specified location.
    pub async fn new(python_bin: PathBuf, script_write_path: PathBuf) -> Result<Self, ScorerError> {
        let mut script_file = tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&script_write_path)
            .await
            .map_err(ScorerError::WriteScript)?;

        script_file
            .write_all(SCORER_SCRIPT.as_bytes())
            .await
            .map_err(ScorerError::WriteScript)?;

        Ok(Scorer {
            script_path: script_write_path,
            python_bin,
        })
    }

    pub async fn score(
        &self,
        true_values: impl AsRef<OsStr>,
        prediction: impl AsRef<OsStr>,
    ) -> Result<f64, ScorerError> {
        let process = tokio::process::Command::new(&self.python_bin)
            .arg(&self.script_path)
            .arg(prediction)
            .arg(true_values)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(ScorerError::StartScript)?;

        // TODO: maybe add a timeout
        let output = process
            .wait_with_output()
            .await
            .map_err(ScorerError::ScriptOutput)?;

        let score: Score = serde_json::from_slice(&output.stdout).map_err(ScorerError::Format)?;

        if let Some(e) = score.error {
            return Err(ScorerError::ScriptError(e));
        }

        Ok(score.score)
    }
}
