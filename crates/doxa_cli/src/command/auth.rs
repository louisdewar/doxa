use clap::ArgMatches;

use serde::{Deserialize, Serialize};

use crate::{
    config::{load_or_default_profile, save_profile},
    error::CommandError,
    request::{post, send_request, send_request_and_parse, Settings},
};

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    auth_token: String,
}

pub async fn login(matches: &ArgMatches, settings: &Settings) -> Result<(), CommandError> {
    let builder = post(settings, "auth/login", true);

    let username: String = matches.value_of("USERNAME").unwrap().into();
    let password = matches.value_of("PASSWORD").unwrap().into();

    let builder = builder.json(&LoginRequest {
        username: username.clone(),
        password,
    });

    let response: LoginResponse = send_request_and_parse(builder).await?;

    let mut profiles = load_or_default_profile(&settings.config_dir).await?;
    profiles.upsert_profile(username.clone(), response.auth_token);

    save_profile(&settings.config_dir, profiles).await?;
    println!("Successfully logged in `{}`", username);

    Ok(())
}

pub async fn register(matches: &ArgMatches, settings: &Settings) -> Result<(), CommandError> {
    let username: String = matches.value_of("USERNAME").unwrap().into();
    let password = matches.value_of("PASSWORD").unwrap().into();

    let invite_code = matches.value_of("INVITE").map(|s| s.to_string());

    let builder = post(
        settings,
        &if let Some(invite) = invite_code {
            format!("auth/invite/accept/{}", invite)
        } else {
            "auth/register".into()
        },
        true,
    );

    // Register and login currently have the same request params
    let builder = builder.json(&LoginRequest {
        username: username.clone(),
        password,
    });

    send_request(builder).await?;

    println!("Successfully registered `{}`", username);

    Ok(())
}
