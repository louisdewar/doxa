use clap::{Parser, Subcommand};

use dialoguer::{theme::ColorfulTheme, Input, Password};
use serde::{Deserialize, Serialize};

use crate::{
    config::{load_or_default_profile, save_profile},
    error::CommandError,
    request::{post, send_request, send_request_and_parse, Settings},
    ui,
};

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Logs in a user and sets the user profile
    Login(LoginArgs),
    /// Registers a new user (optionally with an invite code)
    Register(RegisterArgs),
    /// Shows info about the currently logged in user
    Info,
}

#[derive(Parser)]
pub struct LoginArgs {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Parser)]
pub struct RegisterArgs {
    #[clap(required = false)]
    username: Option<String>,
    #[clap(required = false)]
    password: Option<String>,
    #[clap(short, long)]
    invite: Option<String>,
}

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    auth_token: String,
}

pub async fn handle_subcommand(
    command: AuthCommands,
    settings: &Settings,
) -> Result<(), CommandError> {
    match command {
        AuthCommands::Login(args) => login(args, settings).await,
        AuthCommands::Register(args) => register(args, settings).await,
        AuthCommands::Info => info(settings).await,
    }
}

pub async fn info(settings: &Settings) -> Result<(), CommandError> {
    #[derive(Deserialize)]
    struct Info {
        username: String,
        admin: bool,
        competitions: Vec<String>,
    }

    let total_steps = 2;
    ui::print_step(
        1,
        total_steps,
        format!(
            "Asking server for information about {}",
            ui::keyword(settings.user_profile.clone()?.name)
        ),
    );

    let info: Info = send_request_and_parse(post(settings, "user/info", false)).await?;

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

pub async fn login(args: LoginArgs, settings: &Settings) -> Result<(), CommandError> {
    let builder = post(settings, "auth/login", true);

    let username = args.username.unwrap_or_else(|| {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .interact_text()
            .unwrap()
    });
    let password = args.password.unwrap_or_else(|| {
        Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password (hidden)")
            .interact()
            .unwrap()
    });

    let builder = builder.json(&LoginRequest {
        username: username.clone(),
        password,
    });

    let total_steps = 3;

    ui::print_step(
        1,
        total_steps,
        format!("Logging in {}", ui::keyword(&username)),
    );
    let response: LoginResponse = send_request_and_parse(builder).await?;

    ui::print_step(
        2,
        total_steps,
        format!("Making {} the default user", ui::keyword(&username)),
    );
    let mut profiles = load_or_default_profile(&settings.config_dir).await?;
    profiles.upsert_profile(username.clone(), response.auth_token);
    profiles.set_default_profile(username.clone());

    ui::print_step(3, total_steps, "Saving the profile");

    save_profile(&settings.config_dir, profiles).await?;

    ui::success("Done");
    Ok(())
}

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
}
