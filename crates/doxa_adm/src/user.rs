use doxa_db::{action, diesel::PgConnection, model::user::User};

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum UserCommands {
    /// Lists all users
    List {},
    /// Admin subcommands
    #[clap(subcommand)]
    Admin(AdminCommands),
}

#[derive(Subcommand)]
pub enum AdminCommands {
    /// Lists all admins
    List {},
    /// Promotes a user to an admin (if the user is already an admin this does nothing)
    Promote(PromoteArgs),
    /// Demotes a user from an admin (if the user is already not an admin this does nothing)
    Demote(DemoteArgs),
}

#[derive(Parser)]
pub struct PromoteArgs {
    /// The user that you want to promote to admin
    username: String,
}

#[derive(Parser)]
pub struct DemoteArgs {
    /// The user that you want to demote from admin
    username: String,
}

pub fn handle_subcommand(command: UserCommands, conn: &PgConnection) {
    match command {
        UserCommands::List {} => list_users(conn),
        UserCommands::Admin(subcommand) => handle_admin_subcommand(subcommand, conn),
    }
}

pub fn handle_admin_subcommand(command: AdminCommands, conn: &PgConnection) {
    match command {
        AdminCommands::List {} => list_admins(conn),
        AdminCommands::Promote(args) => set_admin_status(args.username, conn, true),
        AdminCommands::Demote(args) => set_admin_status(args.username, conn, true),
    }
}

pub fn print_user_table(users: &[User]) {
    print_user_table_header();

    for user in users {
        print_user_row(user);
    }
}

fn print_user_table_header() {
    println!("ID USERNAME IS_ADMIN");
}

fn print_user_row(user: &User) {
    println!("{} {} {}", user.id, user.username, user.admin);
}

fn print_single_user(user: &User) {
    print_user_table_header();
    print_user_row(user);
}

pub fn list_users(conn: &PgConnection) {
    let users = action::user::list_users(conn).unwrap();

    print_user_table(&users);
}

pub fn list_admins(conn: &PgConnection) {
    let users = action::user::list_admins(conn).unwrap();

    print_user_table(&users);
}

pub fn set_admin_status(username: String, conn: &PgConnection, admin_status: bool) {
    let user = action::user::set_admin_status(conn, username, admin_status).unwrap();

    print_single_user(&user);
}
