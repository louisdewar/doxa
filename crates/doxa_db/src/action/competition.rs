use crate::model::competition::{Competition, Enrollment, InsertableCompetition};
use crate::{schema as s, DieselError};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

pub fn register_competition(
    conn: &PgConnection,
    competition: &InsertableCompetition,
) -> Result<Competition, DieselError> {
    diesel::insert_into(s::competitions::table)
        .values(competition)
        .get_result(conn)
}

pub fn enroll_user(conn: &PgConnection, enrollment: &Enrollment) -> Result<(), DieselError> {
    diesel::insert_into(s::enrollment::table)
        .values(enrollment)
        .execute(conn)
        .map(|rows| assert_eq!(rows, 1))
}

pub fn get_competition_by_name(
    conn: &PgConnection,
    name: String,
) -> Result<Option<Competition>, DieselError> {
    s::competitions::table
        .filter(s::competitions::columns::name.eq(name))
        .first(conn)
        .optional()
}
