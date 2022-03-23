use std::{ffi::OsStr, path::PathBuf, process::Stdio};

use doxa_competition::{
    tokio::{self, io::AsyncWriteExt},
    tracing::warn,
};
use serde::Deserialize;
use serde_json::Value as JsonValue;

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
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum GroupResult {
    Success {
        score: f64,
        images: Vec<String>,
        #[serde(default)]
        metrics: JsonValue,
        #[serde(default)]
        sequences: JsonValue,
    },
    Failure {
        error: String,
        #[serde(default)]
        forfeit: Option<String>,
    },
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
    ) -> Result<(f64, Vec<String>, JsonValue, JsonValue), ScorerError> {
        let process = tokio::process::Command::new(&self.python_bin)
            .arg(&self.script_path)
            .arg(prediction)
            .arg(true_values)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(ScorerError::StartScript)?;

        // TODO: maybe add a timeout
        let output = process
            .wait_with_output()
            .await
            .map_err(ScorerError::ScriptOutput)?;

        if !output.stderr.is_empty() {
            warn!(stderr=%String::from_utf8_lossy(&output.stderr) , "scorer has stderr");
        }

        let result: GroupResult =
            serde_json::from_slice(&output.stdout).map_err(ScorerError::Format)?;

        match result {
            GroupResult::Failure {
                error,
                forfeit: None,
            } => Err(ScorerError::InternalScriptError(error)),
            GroupResult::Failure {
                error,
                forfeit: Some(forfeit),
            } => Err(ScorerError::ForfeitError { error, forfeit }),
            GroupResult::Success {
                score,
                images,
                metrics,
                sequences,
            } => Ok((score, images, metrics, sequences)),
        }
    }
}
