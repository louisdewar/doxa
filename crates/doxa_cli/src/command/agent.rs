use std::path::PathBuf;

use clap::ArgMatches;
use reqwest::multipart::{Form, Part};

use serde::{Deserialize, Serialize};

use crate::{
    error::CommandError,
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
        "upload" => upload(subcommand.1, &settings, competition_name).await,
        _ => panic!("unrecognised subcommand"),
    }
}

pub async fn upload(
    matches: &ArgMatches,
    settings: &Settings,
    competition_name: String,
) -> Result<(), CommandError> {
    let file_path = PathBuf::from(matches.value_of("FILE").unwrap());

    let form = Form::new().part(
        "file",
        Part::bytes(tokio::fs::read(file_path.clone()).await?)
            .file_name(file_path.file_name().unwrap().to_str().unwrap().to_owned()),
    );

    let builder = post(
        settings,
        format!("storage/upload/{}", competition_name),
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
