use crate::model::game as model;

use crate::{schema as s, DieselError};
use diesel::{PgConnection, RunQueryDsl};

pub fn create_game(
    conn: &PgConnection,
    game: &model::InsertableGame,
) -> Result<model::Game, DieselError> {
    diesel::insert_into(s::games::table)
        .values(game)
        .get_result(conn)
}

pub fn add_participant(
    conn: &PgConnection,
    participant: &model::GameParticipant,
) -> Result<model::GameParticipant, DieselError> {
    diesel::insert_into(s::game_participants::table)
        .values(participant)
        .get_result(conn)
}

pub fn add_event(
    conn: &PgConnection,
    participant: &model::GameParticipant,
) -> Result<model::GameParticipant, DieselError> {
    diesel::insert_into(s::game_participants::table)
        .values(participant)
        .get_result(conn)
}
