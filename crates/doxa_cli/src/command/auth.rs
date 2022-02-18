use std::time::Duration;

use chrono::{DateTime, Utc};
use clap::Subcommand;

use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

use crate::{
    config::{load_or_default_profile, save_profile},
    error::{CommandError, DelegatedAuthTimeout},
    request::{post, send_request_and_parse, Settings},
    ui,
};

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Logs in a user and sets the user profile.
    Login,
    // /// Registers a new user (optionally with an invite code)
    //Register,
    /// Shows info about the currently logged in user
    Info,
}

// TODO: allow using the old username/password auth
// #[derive(Parser)]
// pub struct LoginArgs {
//     username: Option<String>,
//     password: Option<String>,
// }
//
// #[derive(Parser)]
// pub struct RegisterArgs {
//     //#[clap(required = false)]
//     username: Option<String>,
//     //#[clap(required = false)]
//     password: Option<String>,
//     #[clap(short, long)]
//     email: Option<String>,
// }

// #[derive(Serialize)]
// struct LoginRequest {
//     username: String,
//     password: String,
// }

// #[derive(Deserialize)]
// struct LoginResponse {
//     auth_token: String,
// }

pub async fn handle_subcommand(
    command: AuthCommands,
    settings: &Settings,
) -> Result<(), CommandError> {
    match command {
        // AuthCommands::Login(args) => login(args, settings).await,
        // AuthCommands::Register(args) => register(args, settings).await,
        AuthCommands::Login => login(settings).await,
        //AuthCommands::Register => register(settings).await,
        AuthCommands::Info => info(settings).await,
    }
}

#[derive(Deserialize)]
struct Info {
    username: String,
    admin: bool,
    competitions: Vec<String>,
}

pub async fn info(settings: &Settings) -> Result<(), CommandError> {
    let total_steps = 2;
    ui::print_step(
        1,
        total_steps,
        format!(
            "Asking server for information about {}",
            ui::keyword(settings.user_profile.clone()?.name)
        ),
    );

    let info: Info = send_request_and_parse(post(settings, "user/info", false).await?).await?;

    ui::print_step(2, total_steps, "Showing user information");

    println!(
        "{}: `{}`\n{}: `{}`\n{}: {}",
        ui::keyword("username"),
        ui::keyword(info.username),
        ui::keyword("admin"),
        ui::keyword(info.admin),
        ui::keyword("competitions"),
        ui::keyword(format!("{:?}", info.competitions))
    );

    Ok(())
}

#[derive(Deserialize)]
struct StartDelegatedResponse {
    verification_code: String,
    verify_url: String,
    auth_secret: String,
    expires: DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum DelegatedAuthCheckResponse {
    Authenticated { auth_token: String },
    Waiting,
}

#[derive(Serialize)]
struct CheckDelegatedRequest {
    pub verification_code: String,
    pub auth_secret: String,
}

pub async fn login(settings: &Settings) -> Result<(), CommandError> {
    let builder = post(settings, "auth/start_delegated", true).await?;

    let total_steps = 6;

    ui::print_step(
        1,
        total_steps,
        "Asking DOXA to start a delegated authentication flow...",
    );

    let start_response: StartDelegatedResponse = send_request_and_parse(builder).await?;

    ui::print_step(
        2,
        total_steps,
        format!(
            "To login enter the following delegated authentication code {} on your account page or go to {}",
            ui::keyword(&start_response.verification_code),
            ui::keyword(&start_response.verify_url)
        ),
    );

    if webbrowser::open(&start_response.verify_url).is_err() {
        ui::warn(format!(
            "Opening '{}' in your local browser failed, please enter the URL manually.",
            start_response.verify_url
        ));
    }

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(100);
    spinner.set_style(ProgressStyle::default_spinner().template(&format!(
        "{} {{spinner:.green.dim.bold}} [{{elapsed_precise}}] {{msg}} ",
        ui::step(3, total_steps)
    )));
    //spinner.set_message("Waiting for you to login...");

    let check_request = CheckDelegatedRequest {
        verification_code: start_response.verification_code,
        auth_secret: start_response.auth_secret,
    };

    spinner.set_message("Waiting for you to login...");

    let auth_token = loop {
        spinner.enable_steady_tick(600);

        if start_response.expires < Utc::now() {
            return Err(DelegatedAuthTimeout.into());
        }

        tokio::time::sleep(Duration::from_secs(5)).await;

        spinner.enable_steady_tick(60);

        let builder = post(settings, "auth/check_delegated", true)
            .await?
            .json(&check_request);

        let check_response: DelegatedAuthCheckResponse = send_request_and_parse(builder).await?;
        match check_response {
            DelegatedAuthCheckResponse::Authenticated { auth_token } => break auth_token,
            DelegatedAuthCheckResponse::Waiting => {}
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    };

    spinner.finish_with_message("Sucessfully logged in");

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(100);
    spinner.set_style(ProgressStyle::default_spinner().template(&format!(
        "{} {{spinner:.green.dim.bold}} [{{elapsed_precise}}] {{msg}} ",
        ui::step(4, total_steps)
    )));
    spinner.set_message("Successfully logged in, getting user info...");

    let access_token = crate::token::authorize(auth_token.clone(), settings).await?;

    let info: Info = send_request_and_parse(
        post(settings, "user/info", true)
            .await?
            .bearer_auth(&access_token.access_token),
    )
    .await?;

    spinner.finish_with_message(format!(
        "Successfully logged in and retrived user info! Welcome {}.",
        ui::keyword(&info.username)
    ));

    let username = info.username;
    ui::print_step(
        5,
        total_steps,
        format!("Making {} the default user", ui::keyword(&username)),
    );
    let mut profiles = load_or_default_profile(&settings.config_dir).await?;
    profiles.upsert_profile(username.clone(), auth_token);
    profiles.set_default_profile(username.clone());

    ui::print_step(6, total_steps, "Saving the profile");

    save_profile(&settings.config_dir, profiles).await?;

    ui::success("Done");
    Ok(())
}

/*
pub async fn register(args: RegisterArgs, settings: &Settings) -> Result<(), CommandError> {
    let username: String = args.username.unwrap();
    let password = args.password.unwrap();

    let invite_code = args.invite;

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
}*/
