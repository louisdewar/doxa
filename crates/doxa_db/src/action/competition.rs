use crate::model::competition::{Competition, Enrollment, InsertableCompetition};
use crate::model::user::User;
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

pub fn enroll_user(
    conn: &PgConnection,
    enrollment: &Enrollment,
) -> Result<Enrollment, DieselError> {
    diesel::insert_into(s::enrollment::table)
        .values(enrollment)
        .get_result(conn)
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

pub fn get_enrollment(
    conn: &PgConnection,
    user_id: i32,
    competition: String,
) -> Result<Option<Enrollment>, DieselError> {
    s::competitions::table
        .inner_join(s::enrollment::table)
        .filter(s::competitions::columns::name.eq(competition))
        .filter(s::enrollment::columns::user_id.eq(user_id))
        .select(s::enrollment::all_columns)
        .first(conn)
        .optional()
}

pub fn list_competitions(conn: &PgConnection) -> Result<Vec<Competition>, DieselError> {
    s::competitions::table.get_results(conn)
}

pub fn list_user_enrollments(
    conn: &PgConnection,
    user_id: i32,
) -> Result<Vec<Competition>, DieselError> {
    s::enrollment::table
        .inner_join(s::competitions::table)
        .filter(s::enrollment::columns::user_id.eq(user_id))
        .select(s::competitions::all_columns)
        .get_results(conn)
}

pub fn list_competition_enrollments(
    conn: &PgConnection,
    competition_id: i32,
) -> Result<Vec<User>, DieselError> {
    s::enrollment::table
        .inner_join(s::users::table)
        .filter(s::enrollment::columns::competition.eq(competition_id))
        .select(s::users::all_columns)
        .get_results(conn)
}
