use doxa_db::{
    action,
    diesel::PgConnection,
    model::user::{generate_invite_id, Invite},
};

use chrono::Utc;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum InviteCommands {
    /// Lists all invites
    List {},
    /// Creates a new invite
    Create(CreateInviteArgs),
}

#[derive(Parser)]
pub struct CreateInviteArgs {
    /// You can optionally specify a required username when creating an account with this invite
    #[clap(short, long)]
    username: Option<String>,
    /// When the invite expires. This accepts times in a human friendly format
    /// e.g. "14 days" or "14d" or "14m" (for 14 minutes), "12M10m" (12 months and 10 minutes).
    #[clap(long)]
    expires: Option<String>,
    /// Specifies a list of competitions that the system will enroll the user in when the invite is
    /// accepted.
    #[clap(long = "enroll", multiple_values = true)]
    enrollments: Option<Vec<String>>,
}

pub fn handle_subcommand(command: InviteCommands, conn: &PgConnection) {
    match command {
        InviteCommands::List {} => list_invites(conn),
        InviteCommands::Create(args) => create_invite(args, conn),
    }
}

// TODO: turn these print methods into a trait
pub fn print_invite_table(invites: &[Invite]) {
    print_invite_table_header();

    for invite in invites {
        print_invite_row(invite);
    }
}

fn print_invite_table_header() {
    println!("ID USERNAME EXPIRES ENROLLMENTS");
}

fn print_invite_row(invite: &Invite) {
    use chrono_humanize::{Accuracy, HumanTime, Tense};

    println!(
        "{} {} {} {:?}",
        invite.id,
        invite.username.as_deref().unwrap_or("-"),
        invite
            .expires_at
            .map(|time| HumanTime::from(time).to_text_en(Accuracy::Rough, Tense::Future))
            .unwrap_or_else(|| "Never".into()),
        invite.enrollments
    );
}

fn print_single_invite(invite: &Invite) {
    print_invite_table_header();
    print_invite_row(invite);
}

pub fn list_invites(conn: &PgConnection) {
    let invites = action::user::list_invites(conn).unwrap();

    print_invite_table(&invites);
}

pub fn create_invite(args: CreateInviteArgs, conn: &PgConnection) {
    let username = args.username;
    let enrollments = args.enrollments.unwrap_or_default();

    let expires_at = args.expires;

    let expires_at = expires_at.as_ref().map(|expires| {
        Utc::now()
            .checked_add_signed(
                chrono::Duration::from_std(parse_duration::parse(expires).unwrap()).unwrap(),
            )
            .unwrap()
    });
    // TODO: validate username (once the server does that too)

    let invite = doxa_db::action::user::create_invite(
        conn,
        Invite {
            id: generate_invite_id(),
            username,
            enrollments,
            expires_at,
        },
    )
    .unwrap();

    print_single_invite(&invite);
}
