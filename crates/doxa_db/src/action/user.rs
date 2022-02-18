use crate::model::user::{self as model};
use crate::{schema as s, DieselError};
use diesel::pg::upsert::excluded;
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

/// Creates a new user if they did not already exist, or updates the values if they did exist
pub fn upsert_user(
    conn: &PgConnection,
    user: &model::InsertableUser,
) -> Result<model::User, DieselError> {
    diesel::insert_into(s::users::table)
        .values(user)
        .on_conflict(s::users::id)
        .do_update()
        .set((
            s::users::username.eq(excluded(s::users::username)),
            s::users::extra.eq(excluded(s::users::extra)),
            s::users::admin.eq(excluded(s::users::admin)),
        ))
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

pub fn get_user_by_id_optional(
    conn: &PgConnection,
    id: i32,
) -> Result<Option<model::User>, DieselError> {
    s::users::table
        .filter(s::users::columns::id.eq(id))
        .first(conn)
        .optional()
}
/// Sets whether or not the user with the specified username is an admin.
pub fn set_admin_status(
    conn: &PgConnection,
    username: String,
    admin: bool,
) -> Result<model::User, DieselError> {
    diesel::update(s::users::dsl::users.filter(s::users::columns::username.eq(username)))
        .set(s::users::columns::admin.eq(admin))
        .get_result(conn)
}

pub fn list_users(conn: &PgConnection) -> Result<Vec<model::User>, DieselError> {
    s::users::table.get_results(conn)
}

pub fn list_admins(conn: &PgConnection) -> Result<Vec<model::User>, DieselError> {
    s::users::table
        .filter(s::users::columns::admin.eq(true))
        .get_results(conn)
}
