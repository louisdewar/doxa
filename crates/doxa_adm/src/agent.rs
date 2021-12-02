use clap::ArgMatches;
use doxa_db::{
    action,
    diesel::PgConnection,
    model::{competition::Competition, storage::AgentUpload, user::User},
};

pub fn agent_subcommand(matches: &ArgMatches, conn: &PgConnection) {
    let subcommand = matches
        .subcommand()
        .expect("missing competition subcommand");
    let sub_matches = subcommand.1;

    match subcommand.0 {
        "list" => list_agents(sub_matches, conn),
        _ => panic!("unrecognised user subcommand"),
    }
}

fn print_agent_table(agents: &[(AgentUpload, User, Competition)]) {
    print_agent_table_header();

    for agent in agents {
        print_agent_row(agent);
    }
}

fn print_agent_table_header() {
    println!("ID OWNER COMPETITION UPLOADED");
}

fn print_agent_row((agent, user, competition): &(AgentUpload, User, Competition)) {
    println!(
        "{} {} {} {}",
        agent.id, user.username, competition.name, agent.uploaded
    );
}

pub fn list_agents(matches: &ArgMatches, conn: &PgConnection) {
    let username = matches.value_of("USERNAME").unwrap();
    let competition_name = matches.value_of("COMPETITION").unwrap();

    let user = action::user::get_user_by_username(conn, username)
        .unwrap()
        .expect("User does not exist");
    let competition = action::competition::get_competition_by_name(conn, competition_name)
        .unwrap()
        .expect("Competition does not exist");

    let uploads = action::storage::list_agents(conn, user.id, competition.id).unwrap();

    print_agent_table(
        &uploads
            .into_iter()
            .map(|agent| (agent, user.clone(), competition.clone()))
            .collect::<Vec<_>>(),
    );
}
