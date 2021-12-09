use std::{io::Write, path::PathBuf};

use flate2::{write::GzEncoder, Compression};
use reqwest::multipart::{Form, Part};

use serde::Deserialize;

use crate::{
    error::{CommandError, UploadError},
    request::{post, send_request_and_parse, Settings},
    ui,
};

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum AgentCommands {
    /// Logs in a user and sets the user profile
    Upload(UploadArgs),
}

#[derive(Parser)]
pub struct UploadArgs {
    /// The name of the competition to upload the agent to.
    competition: String,
    /// The path to the directory of the agent or a tar file (ending with `.tar` or `.tar.gz`) to upload
    agent_path: PathBuf,
}

#[derive(Deserialize)]
struct UploadResponse {
    competition: String,
    id: String,
}

pub async fn handle_subcommand(
    command: AgentCommands,
    settings: &Settings,
) -> Result<(), CommandError> {
    match command {
        AgentCommands::Upload(args) => upload(args, settings).await,
    }
}

pub async fn upload(args: UploadArgs, settings: &Settings) -> Result<(), CommandError> {
    let competition_name = args.competition;
    let agent_path = args.agent_path;
    let meta = tokio::fs::metadata(&agent_path)
        .await
        .map_err(UploadError::ReadAgentError)?;

    let agent_file_name = agent_path.file_name().unwrap().to_str().unwrap().to_owned();

    let total_steps = 4;
    ui::step(1, total_steps, "Finding the agent");

    let (file_name, data) = if meta.is_dir() {
        tokio::fs::metadata(agent_path.join("doxa.yaml"))
            .await
            .map_err(|_| UploadError::MissingExecutionConfig)?;
        let agent_path = agent_path.clone();

        ui::step(2, total_steps, "Creating a tar archive of the agent");
        let data = tokio::task::spawn_blocking(move || {
            let mut tar = tar::Builder::new(Vec::new());
            tar.append_dir_all(".", agent_path)?;
            let bytes = tar.into_inner()?;

            let mut e = GzEncoder::new(Vec::new(), Compression::default());
            e.write_all(&bytes).unwrap();

            e.finish()
        })
        .await
        .unwrap()
        .map_err(UploadError::ReadAgentError)?;

        (format!("{}.tar.gz", agent_file_name), data)
    } else {
        if !(agent_path.ends_with(".tar.gz") || agent_path.ends_with(".tar")) {
            return Err(UploadError::IncorrectExtension.into());
        }

        ui::step(2, total_steps, "Reading agent tar file");
        (agent_file_name, tokio::fs::read(agent_path.clone()).await?)
    };

    let form = Form::new().part("file", Part::bytes(data).file_name(file_name));

    ui::step(3, total_steps, "Uploading agent");
    let builder = post(
        settings,
        &format!("storage/upload/{}", competition_name),
        false,
    )
    .multipart(form);

    let response: UploadResponse = send_request_and_parse(builder).await?;

    ui::step(
        4,
        total_steps,
        format!(
            "Successfully uploaded agent to competition {}, it was given the id {}",
            ui::keyword(response.competition),
            ui::keyword(response.id)
        ),
    );

    Ok(())
}
