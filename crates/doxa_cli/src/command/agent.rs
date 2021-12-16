use std::path::PathBuf;

use flate2::{write::GzEncoder, Compression};
use indicatif::{HumanBytes, HumanDuration, ProgressBar, ProgressStyle};
use reqwest::multipart::{Form, Part};

use futures_util::StreamExt;
use serde::Deserialize;
use tokio::io::{AsyncSeekExt, BufStream};
use tokio_util::codec::{BytesCodec, FramedRead};

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

    let agent_file_name = agent_path
        .canonicalize()?
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let total_steps = 4;
    ui::print_step(1, total_steps, "Finding the agent");

    let (file_name, agent_tar_file) = if meta.is_dir() {
        tokio::fs::metadata(agent_path.join("doxa.yaml"))
            .await
            .map_err(|_| UploadError::MissingExecutionConfig)?;
        let agent_path = agent_path.clone();

        //ui::print_step(2, total_steps, "Creating a tar archive of the agent");
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(100);
        spinner.set_style(ProgressStyle::default_spinner().template(&format!(
            "{} {{spinner:.green.dim.bold}} [{{elapsed_precise}}] {{msg}}",
            ui::step(2, total_steps)
        )));
        spinner.set_message("Creating a tar archive of the agent");

        let tar_file = tokio::task::spawn_blocking(move || {
            let mut tmpfile = tempfile::tempfile().unwrap();
            let writer = GzEncoder::new(&mut tmpfile, Compression::default());

            let mut tar = tar::Builder::new(writer);
            tar.append_dir_all(".", agent_path)?;

            tar.into_inner()?;

            Ok(tmpfile)
        })
        .await
        .unwrap()
        .map_err(UploadError::ReadAgentError)?;

        let mut tar_file = tokio::fs::File::from_std(tar_file);

        let file_len = tar_file.metadata().await?.len();
        let elapsed = spinner.elapsed();
        drop(spinner);
        ui::print_step(
            2,
            total_steps,
            format!(
                "Created tar archive of agent ({}) in {}",
                HumanBytes(file_len),
                HumanDuration(elapsed)
            ),
        );

        tar_file.seek(std::io::SeekFrom::Start(0)).await?;

        (format!("{}.tar.gz", agent_file_name), tar_file)
    } else {
        if !(agent_path.ends_with(".tar.gz") || agent_path.ends_with(".tar")) {
            return Err(UploadError::IncorrectExtension.into());
        }

        ui::print_step(2, total_steps, "Reading agent tar file");
        //(agent_file_name, tokio::fs::read(agent_path.clone()).await?)
        (
            agent_file_name,
            tokio::fs::OpenOptions::new()
                .read(true)
                .open(agent_path.clone())
                .await?,
        )
    };

    let file_len = agent_tar_file.metadata().await?.len();

    let upload_bar = ProgressBar::new(file_len);
    upload_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    let mut uploaded = 0;

    let mut stream = FramedRead::new(BufStream::new(agent_tar_file), BytesCodec::new());

    let file_stream = async_stream::stream! {
        while let Some(chunk) = stream.next().await {
            if let Ok(chunk) = &chunk {
                uploaded += chunk.len() as u64;
                upload_bar.set_position(uploaded);

                if (uploaded >= file_len) {
                    upload_bar.finish_with_message("uploaded");
                }
            }

            yield chunk;
        }
    };

    let form = Form::new().part(
        "file",
        Part::stream_with_length(reqwest::Body::wrap_stream(file_stream), file_len)
            .file_name(file_name),
    );

    ui::print_step(3, total_steps, "Uploading agent");

    let builder = post(
        settings,
        &format!("storage/upload/{}", competition_name),
        false,
    )
    .multipart(form);

    let response: UploadResponse = send_request_and_parse(builder).await?;

    ui::print_step(
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
