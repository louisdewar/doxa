use doxa_core::{handle_doxa_error, RespondableError};
use doxa_db::PgPool;

use std::future::Future;
use std::pin::Pin;

use actix_web::{dev, web, FromRequest, HttpRequest, HttpResponse};

use crate::{
    error::{InvalidAuthentication, MissingAuthentication},
    guard::{AuthGuard, AuthGuardPart},
};

const MATTER_TOKEN_HEADER: &'static str = "X-Matter-Auth";

impl<T: AuthGuardPart> FromRequest for AuthGuard<T> {
    type Error = HttpResponse;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<PgPool>>().unwrap().clone();

        let auth_header = req
            .headers()
            .get(MATTER_TOKEN_HEADER)
            .map(|h| h.to_str().map(|s| s.to_owned()));

        Box::pin(async move {
            let auth_header = match auth_header {
                Some(Ok(val)) => {
                    if val.len() < 8 || &val[0..7] != "Bearer " {
                        return Err(InvalidAuthentication.into_response());
                    }
                    val
                }
                Some(Err(_)) => return Err(InvalidAuthentication.into_response()),
                None => return Err(MissingAuthentication.into_response()),
            };

            let token = handle_doxa_error!(
                web::block(move || {
                    let conn = pool.get().unwrap();
                    action::get_token(&conn, &auth_header)
                })
                .await
            );

            if let Some(token) = token {
                Ok(token)
            } else {
                return Err(HttpResponse::BadRequest()
                    .json(json!({
                        "error": "The token doesn't exist or is invalid",
                        "ERROR_CODE": "TOKEN_NOT_FOUND"
                    }))
                    .into());
            }
        })
    }
}
