use clap::ArgMatches;
use doxa_db::{action, diesel::PgConnection, model::user::User};

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

pub fn list_users(_matches: &ArgMatches, conn: &PgConnection) {
    let users = action::user::list_users(conn).unwrap();

    print_user_table(&users);
}

pub fn list_admins(_matches: &ArgMatches, conn: &PgConnection) {
    let users = action::user::list_admins(conn).unwrap();

    print_user_table(&users);
}

pub fn set_admin_status(matches: &ArgMatches, conn: &PgConnection, admin_status: bool) {
    let username = matches.value_of("USERNAME").unwrap();
    let user = action::user::set_admin_status(conn, username.to_string(), admin_status).unwrap();

    print_single_user(&user);
}
