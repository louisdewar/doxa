use clap::{ArgEnum, Parser};
use serde_json::Value;
use std::error::Error;

use crate::{
    error::CommandError,
    request::{get, post, send_request_and_parse, Settings},
};

#[derive(Parser)]
pub struct RawCommand {
    endpoint: String,
    #[clap(arg_enum)]
    request_type: RequestType,
    #[clap(long)]
    never_auth: bool,
    #[clap(parse(try_from_str = parse_key_val), multiple_occurrences(true))]
    body: Vec<(String, Value)>,
}

#[derive(ArgEnum, Clone)]
pub enum RequestType {
    Post,
    Get,
}

// NOTE: this code is adapted from the clap docs
/// Parse a single key-value pair
fn parse_key_val(s: &str) -> Result<(String, Value), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    let value: Value = serde_json::from_str(&s[pos + 1..])?;
    Ok((s[..pos].to_string(), value))
}

pub async fn handle_subcommand(
    command: RawCommand,
    settings: &Settings,
) -> Result<(), CommandError> {
    let mut request_body = serde_json::Map::new();

    for (key, value) in command.body {
        request_body.insert(key, value);
    }

    let request = match command.request_type {
        RequestType::Get => get(settings, &command.endpoint, command.never_auth).await?,
        RequestType::Post => post(settings, &command.endpoint, command.never_auth).await?,
    };

    println!(
        "Request body:\n{}",
        serde_json::to_string_pretty(&request_body).unwrap()
    );

    let response: Value = send_request_and_parse(request.json(&request_body)).await?;

    println!(
        "Got response:\n{}",
        serde_json::to_string_pretty(&response).unwrap()
    );

    Ok(())
}
