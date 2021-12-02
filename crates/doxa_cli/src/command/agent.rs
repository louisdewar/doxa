use std::{io::Write, path::PathBuf};

use clap::ArgMatches;
use flate2::{write::GzEncoder, Compression};
use reqwest::multipart::{Form, Part};

use serde::Deserialize;

use crate::{
    error::{CommandError, UploadError},
    request::{post, send_request_and_parse, Settings},
};

#[derive(Deserialize)]
struct UploadResponse {
    competition: String,
    id: String,
}

pub async fn subcommand(matches: &ArgMatches, settings: &Settings) -> Result<(), CommandError> {
    let subcommand = matches.subcommand().expect("missing agent subcommand");
    let competition_name: String = matches.value_of("COMPETITION_NAME").unwrap().into();
    match subcommand.0 {
        "upload" => upload(subcommand.1, settings, competition_name).await,
        _ => panic!("unrecognised subcommand"),
    }
}

pub async fn upload(
    matches: &ArgMatches,
    settings: &Settings,
    competition_name: String,
) -> Result<(), CommandError> {
    let agent_path = PathBuf::from(matches.value_of("PATH").unwrap());
    let meta = tokio::fs::metadata(&agent_path)
        .await
        .map_err(UploadError::ReadAgentError)?;

    let agent_file_name = agent_path.file_name().unwrap().to_str().unwrap().to_owned();

    let (file_name, data) = if meta.is_dir() {
        tokio::fs::metadata(agent_path.join("doxa.yaml"))
            .await
            .map_err(|_| UploadError::MissingExecutionConfig)?;
        let agent_path = agent_path.clone();

        println!("Creating tar archive of agent");
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
        println!("Finished creating tar archive of agent");

        (format!("{}.tar.gz", agent_file_name), data)
    } else {
        if !(agent_path.ends_with(".tar.gz") || agent_path.ends_with(".tar")) {
            return Err(UploadError::IncorrectExtension.into());
        }

        (agent_file_name, tokio::fs::read(agent_path.clone()).await?)
    };

    let form = Form::new().part("file", Part::bytes(data).file_name(file_name));

    let builder = post(
        settings,
        &format!("storage/upload/{}", competition_name),
        false,
    )
    .multipart(form);

    let response: UploadResponse = send_request_and_parse(builder).await?;

    println!(
        "Successfully uploaded agent to competition `{}`, it was given the id `{}`",
        response.competition, response.id
    );

    Ok(())
}
