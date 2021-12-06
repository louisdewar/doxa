use clap::ArgMatches;
use doxa_db::{
    action,
    diesel::PgConnection,
    model::competition::{Competition, Enrollment, InsertableCompetition},
    was_unique_key_violation,
};

pub fn competition_subcommand(matches: &ArgMatches, conn: &PgConnection) {
    let subcommand = matches
        .subcommand()
        .expect("missing competition subcommand");
    let sub_matches = subcommand.1;

    match subcommand.0 {
        "list" => list_competitions(sub_matches, conn),
        "enroll" => enroll(sub_matches, conn),
        "unenroll" => unenroll(sub_matches, conn),
        "create" => create(sub_matches, conn),
        "list_user_enrollments" => list_user_enrollments(sub_matches, conn),
        "list_competition_enrollments" => list_competition_enrollments(sub_matches, conn),
        _ => panic!("unrecognised user subcommand"),
    }
}

fn print_competition_table(competitions: &[Competition]) {
    print_competition_table_header();

    for competition in competitions {
        print_competition_row(competition);
    }
}

fn print_competition_table_header() {
    println!("ID NAME");
}

fn print_competition_row(competition: &Competition) {
    println!("{} {}", competition.id, competition.name);
}

fn print_single_competition(competition: &Competition) {
    print_competition_table_header();
    print_competition_row(competition);
}

pub fn list_competitions(_matches: &ArgMatches, conn: &PgConnection) {
    let competitions = action::competition::list_competitions(conn).unwrap();

    print_competition_table(&competitions);
}

pub fn enroll(matches: &ArgMatches, conn: &PgConnection) {
    let username = matches.value_of("USERNAME").unwrap();
    let competition_name = matches.value_of("COMPETITION").unwrap();

    let user = action::user::get_user_by_username(conn, username)
        .unwrap()
        .expect("User does not exist");
    let competition = action::competition::get_competition_by_name(conn, competition_name)
        .unwrap()
        .expect("Competition does not exist");

    let enrollment: Enrollment = action::competition::enroll_user(
        conn,
        &Enrollment {
            user_id: user.id,
            competition: competition.id,
        },
    )
    .unwrap();

    assert_eq!(enrollment.competition, competition.id);
    assert_eq!(enrollment.user_id, user.id);

    println!(
        "User (id={}, username={}) is now enrolled in competition (id={},name={})",
        enrollment.user_id, username, enrollment.competition, competition.name
    );
}

pub fn unenroll(matches: &ArgMatches, _conn: &PgConnection) {
    let username = matches.value_of("USERNAME").unwrap();
    let competition_name = matches.value_of("COMPETITION").unwrap();

    unimplemented!("unenroll {} from {}", username, competition_name);
}

pub fn create(matches: &ArgMatches, conn: &PgConnection) {
    let competition_name = matches.value_of("COMPETITION_NAME").unwrap();

    let ensure_exists = matches.is_present("ENSURE_EXISTS");

    let competition = match action::competition::register_competition(
        conn,
        &InsertableCompetition {
            name: competition_name.to_string(),
        },
    ) {
        Ok(competition) => competition,
        Err(e) if was_unique_key_violation(&e) && ensure_exists => {
            println!("The competition already exists and --ensure-exists was set so ignoring");
            return;
        }
        Err(e) => panic!("failed to create competition: {}", e),
    };

    print_single_competition(&competition);
}

pub fn list_user_enrollments(matches: &ArgMatches, conn: &PgConnection) {
    let username = matches.value_of("USERNAME").unwrap();
    let user = action::user::get_user_by_username(conn, username)
        .unwrap()
        .expect("User does not exist");

    let competitions = action::competition::list_user_enrollments(conn, user.id).unwrap();

    print_competition_table(&competitions);
}

pub fn list_competition_enrollments(matches: &ArgMatches, conn: &PgConnection) {
    let competition_name = matches.value_of("COMPETITION").unwrap();
    let competition = action::competition::get_competition_by_name(conn, competition_name)
        .unwrap()
        .expect("Competition does not exist");

    let users = action::competition::list_competition_enrollments(conn, competition.id).unwrap();

    crate::user::print_user_table(&users);
}
