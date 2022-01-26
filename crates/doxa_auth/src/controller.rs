use std::time::Duration;

use actix_web::{web, HttpResponse};
use diesel::PgConnection;
use doxa_core::{autha_client::flow::FlowResponse, EndpointResult};
use doxa_db::{model::competition::Enrollment, PgPool};
use hmac::Hmac;
use sha2::Sha256;

use crate::{
    error::{CheckEnrollmentError, CompetitionNotFound, UpsertUserError},
    route::response,
    token::{generate_jwt, Token},
};

use crate::AuthaUser;

use doxa_db::{action, model, model::user::User};

// 1 week
pub const JWT_LIFE: Duration = Duration::from_secs(60 * 60 * 24 * 7);

pub const TOKEN_GENERATION_BYTES: usize = 5;

/// A token generation is a string that is required to be present in auth tokens.
/// Whenever an auth token is used the token generation must be checked to make sure it matches the
/// one in the database.
/// This means that if the token generation is updated to a new value in the database it can be
/// used to invalidate all active auth tokens.
fn new_token_generation() -> String {
    use rand::Rng;

    let generation: Vec<u8> = rand::thread_rng()
        .sample_iter(rand::distributions::Standard)
        .take(TOKEN_GENERATION_BYTES)
        .collect();

    base64::encode(generation)
}

pub fn upsert_user(conn: &PgConnection, user: AuthaUser) -> Result<User, UpsertUserError> {
    let token_generation = new_token_generation();
    let user = model::user::InsertableUser {
        id: user.id,
        extra: user.extra,
        username: user.username,
        token_generation,
    };

    let user = action::user::upsert_user(conn, &user)?;

    Ok(user)
}

pub fn generate_new_jwt_token(user: &User, key: &Hmac<Sha256>) -> String {
    let token = Token::new_with_duration(user.id, user.token_generation.clone(), JWT_LIFE);

    generate_jwt(&token, key)
}

pub fn process_authenticated_user(
    db_pool: &PgPool,
    user: AuthaUser,
    key: &Hmac<Sha256>,
) -> Result<String, UpsertUserError> {
    let user = upsert_user(&db_pool.get().unwrap(), user)?;

    Ok(generate_new_jwt_token(&user, key))
}

pub async fn handle_flow_response(
    settings: &crate::Settings,
    db_pool: web::Data<PgPool>,
    response: FlowResponse,
) -> EndpointResult {
    let jwt_secret = settings.jwt_secret.clone();
    let response = match response {
        FlowResponse::Authenticated { user } => {
            let jwt = web::block(move || process_authenticated_user(&db_pool, user, &jwt_secret))
                .await??;

            response::ProviderFlow::Authenticated { auth_token: jwt }
        }
        FlowResponse::Incomplete { payload } => response::ProviderFlow::Incomplete { payload },
    };

    Ok(HttpResponse::Ok().json(response))
}

/// If the user is enrolled then this returns `Ok(enrollment)` containing the enrollment
/// In any other case (including both that the user is not enrolled or there has been
/// some internal error with the database) an error is returned
///
/// TEMPORARILY THIS CURRENTLY ALWAYS RETURNS AN ENROLLMENT EVEN IF THE USER IS NOT ACTUALLY
/// ENROLLED
pub fn is_enrolled(
    conn: &PgConnection,
    user_id: i32,
    competition: String,
) -> Result<Enrollment, CheckEnrollmentError> {
    // TODO: properly fix.
    // Currently this DOES NOT PERFORM AN ENROLLMENT CHECK.
    // This is to help with some convenience issues until the enrollment process is improved.
    let competition = action::competition::get_competition_by_name(conn, &competition)?
        .ok_or(CompetitionNotFound)?;

    Ok(Enrollment {
        user_id,
        competition: competition.id,
    })

    //action::competition::get_competition_by_name(conn, &competition)?.ok_or(CompetitionNotFound)?;
    //Ok(action::competition::get_enrollment(conn, user_id, competition)?.ok_or(UserNotEnrolled)?)
}
