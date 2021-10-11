use crate::model::user::{self as model, Invite};
use crate::{schema as s, DieselError};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

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

pub fn create_invite(conn: &PgConnection, invite: Invite) -> Result<Invite, DieselError> {
    diesel::insert_into(s::invites::table)
        .values(invite)
        .get_result(conn)
}

pub fn remove_invite(conn: &PgConnection, id: String) -> Result<Option<Invite>, DieselError> {
    diesel::delete(s::invites::table.filter(s::invites::id.eq(id)))
        .get_result(conn)
        .optional()
}

pub fn get_invite(conn: &PgConnection, id: String) -> Result<Option<Invite>, DieselError> {
    s::invites::table
        .filter(s::invites::columns::id.eq(id))
        .first(conn)
        .optional()
}

pub fn list_invites(conn: &PgConnection) -> Result<Vec<Invite>, DieselError> {
    s::invites::table.get_results(conn)
}
