use crate::model;
use diesel::PgConnection;

pub fn create_user(
    conn: &PgConnection,
    user: &model::InsertableUser,
) -> Result<model::User, DieselError> {
    diesel::insert_into(s::users::table)
        .values(user)
        .get_result(conn)
}

pub fn get_user_by_username(
    conn: &PgConnection,
    username: &str,
) -> Result<Option<model::User>, DieselError> {
    s::users::table
        .filter(s::users::columns::username.eq(username))
        .first(conn)
        .optional()
}

/// This method does not return an `Option` of the user because if you have the user id,
/// it's likely you got that from doing a recent database query so it doesn't make sense for the
/// user to not exist.
/// If they don't exist it will return an error which should be handled just as any other kind of
/// database error.
pub fn get_user_by_id(conn: &PgConnection, id: i32) -> Result<model::User, DieselError> {
    s::users::table
        .filter(s::users::columns::id.eq(id))
        .first(conn)
}
