use clap::ArgMatches;
use doxa_db::{
    action,
    diesel::PgConnection,
    model::user::{generate_invite_id, Invite},
};

use chrono::Utc;

pub fn invite_subcommand(matches: &ArgMatches, conn: &PgConnection) {
    let subcommand = matches
        .subcommand()
        .expect("missing competition subcommand");
    let sub_matches = subcommand.1;

    match subcommand.0 {
        "list" => list_invites(sub_matches, conn),
        "create" => create_invite(sub_matches, conn),
        _ => panic!("unrecognised user subcommand"),
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

pub fn list_invites(_matches: &ArgMatches, conn: &PgConnection) {
    let invites = action::user::list_invites(conn).unwrap();

    print_invite_table(&invites);
}

pub fn create_invite(matches: &ArgMatches, conn: &PgConnection) {
    let username = matches.value_of("USERNAME").map(|s| s.to_string());
    let enrollments = matches
        .values_of_t::<String>("ENROLLMENTS")
        .unwrap_or_default();

    let expires_at = matches.value_of("EXPIRES_AT");

    let expires_at = expires_at.map(|expires| {
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
