use clap::{Parser, Subcommand};
use doxa_db::{
    action,
    diesel::PgConnection,
    model::competition::{Competition, Enrollment, InsertableCompetition},
    was_unique_key_violation,
};

#[derive(Subcommand)]
pub enum CompetitionCommands {
    /// Lists all competitions
    List {},
    /// Enrolls a user in a competition
    Enroll(EnrollArgs),
    /// Unenrolls a user from a competition
    Unenroll(EnrollArgs),
    /// Lists all the competitions that a user is enrolled in
    ListUserEnrollments { username: String },
    /// Lists all the users enrolled in a competition
    ListCompetitionEnrollments { competition_name: String },
    /// Creates a competition with a specific name
    Create(CreateCompetitonArgs),
}

#[derive(Parser)]
pub struct EnrollArgs {
    username: String,
    competition_name: String,
}

#[derive(Parser)]
pub struct CreateCompetitonArgs {
    /// The name of the competition to create
    competition_name: String,
    #[clap(long)]
    /// If the competition already exists and this flag is set, then this will **not** return an error.
    ensure_exists: bool,
}

pub fn handle_subcommand(subcommand: CompetitionCommands, conn: &PgConnection) {
    match subcommand {
        CompetitionCommands::List {} => list_competitions(conn),
        CompetitionCommands::Enroll(args) => enroll(args, conn),
        CompetitionCommands::Unenroll(args) => unenroll(args, conn),
        CompetitionCommands::ListUserEnrollments { username } => {
            list_user_enrollments(username, conn)
        }
        CompetitionCommands::ListCompetitionEnrollments { competition_name } => {
            list_competition_enrollments(competition_name, conn)
        }
        CompetitionCommands::Create(args) => create(args, conn),
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

pub fn list_competitions(conn: &PgConnection) {
    let competitions = action::competition::list_competitions(conn).unwrap();

    print_competition_table(&competitions);
}

pub fn enroll(args: EnrollArgs, conn: &PgConnection) {
    let username = args.username;
    let competition_name = args.competition_name;

    let user = action::user::get_user_by_username(conn, &username)
        .unwrap()
        .expect("User does not exist");
    let competition = action::competition::get_competition_by_name(conn, &competition_name)
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

pub fn unenroll(args: EnrollArgs, _conn: &PgConnection) {
    let username = args.username;
    let competition_name = args.competition_name;

    unimplemented!("unenroll {} from {}", username, competition_name);
}

pub fn create(args: CreateCompetitonArgs, conn: &PgConnection) {
    let competition_name = args.competition_name;
    let ensure_exists = args.ensure_exists;

    let competition = match action::competition::register_competition(
        conn,
        &InsertableCompetition {
            name: competition_name,
        },
    ) {
        Ok(competition) => competition,
        Err(e) if was_unique_key_violation(&e) && ensure_exists => {
            eprintln!("The competition already exists and --ensure-exists was set so ignoring");
            return;
        }
        Err(e) => panic!("failed to create competition: {}", e),
    };

    print_single_competition(&competition);
}

pub fn list_user_enrollments(username: String, conn: &PgConnection) {
    let user = action::user::get_user_by_username(conn, &username)
        .unwrap()
        .expect("User does not exist");

    let competitions = action::competition::list_user_enrollments(conn, user.id).unwrap();

    print_competition_table(&competitions);
}

pub fn list_competition_enrollments(competition_name: String, conn: &PgConnection) {
    let competition = action::competition::get_competition_by_name(conn, &competition_name)
        .unwrap()
        .expect("Competition does not exist");

    let users = action::competition::list_competition_enrollments(conn, competition.id).unwrap();

    crate::user::print_user_table(&users);
}
