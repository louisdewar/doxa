use std::time::Duration;

use diesel::PgConnection;
use doxa_db::{model::competition::Enrollment, was_unique_key_violation};
use hmac::Hmac;
use sha2::Sha256;

use crate::{
    error::{
        CheckEnrollmentError, CompetitionNotFound, CreateUserError, IncorrectPassword, LoginError,
        UserAlreadyExists, UserNotEnrolled, UserNotFound,
    },
    password,
    token::{generate_jwt, Token},
};

use doxa_db::{action, model, model::user::User};

// 1 week
pub const JWT_LIFE: u64 = 60 * 60 * 24 * 7;

/// Creates a user with a given username / password, hashing the password before inserting it into
/// the database.
///
/// This method is blocking.
/// TODO: maybe this method should return the token?
pub fn create_user(
    conn: &PgConnection,
    username: String,
    password: &str,
) -> Result<User, CreateUserError> {
    // TODO: do some checking of username, e.g. no spaces, certain length, maybe limit characters
    // to ascii?

    let password = password::new_hashed(&password);
    let user = model::user::InsertableUser { username, password };

    action::user::create_user(conn, &user).map_err(|e| {
        if was_unique_key_violation(&e) {
            UserAlreadyExists.into()
        } else {
            e.into()
        }
    })
}

/// Verifies the given username / password and returns a JWT if successfull.
///
/// This method is blocking.
pub fn login(
    conn: &PgConnection,
    jwt_key: &Hmac<Sha256>,
    username: &str,
    password: &str,
) -> Result<String, LoginError> {
    let user = if let Some(user) = action::user::get_user_by_username(conn, username)? {
        user
    } else {
        return Err(UserNotFound.into());
    };

    if !password::verify(password, &user.password) {
        return Err(IncorrectPassword.into());
    }

    Ok(generate_jwt(
        &Token::new_with_duration(user.id, Duration::from_secs(JWT_LIFE)),
        &jwt_key,
    ))
}

/// If the user is enrolled then this returns `Ok(enrollment)` containing the enrollment
/// In any other case (including both that the user is not enrolled or there has been
/// some internal error with the database) an error is returned
pub fn is_enrolled(
    conn: &PgConnection,
    user_id: i32,
    competition: String,
) -> Result<Enrollment, CheckEnrollmentError> {
    action::competition::get_competition_by_name(conn, competition.clone())?
        .ok_or(CompetitionNotFound)?;

    Ok(action::competition::get_enrollment(conn, user_id, competition)?.ok_or(UserNotEnrolled)?)
}
