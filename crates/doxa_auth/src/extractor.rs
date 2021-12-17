use doxa_core::{error::RespondableErrorWrapper, tokio};
use doxa_db::{was_unique_key_violation, PgPool};

use std::future::Future;
use std::pin::Pin;

use actix_web::{dev, web, FromRequest, HttpRequest};

use crate::{
    error::{
        IncorrectTokenGeneration, InvalidAuthenticationHeader, MissingAuthentication, UserNotFound,
    },
    guard::AuthGuard,
    settings::Settings,
};

impl FromRequest for AuthGuard<()> {
    type Error = RespondableErrorWrapper;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<PgPool>>().unwrap().clone();
        let settings = req.app_data::<web::Data<Settings>>().unwrap().clone();

        let auth_header = req
            .headers()
            .get("Authorization")
            .map(|h| h.to_str().map(|s| s.to_owned()));

        Box::pin(async move {
            let auth_header = match auth_header {
                Some(Ok(val)) => {
                    if val.len() < 8 || &val[0..7] != "Bearer " {
                        return Err(InvalidAuthenticationHeader.into());
                    }
                    val
                }
                Some(Err(_)) => return Err(InvalidAuthenticationHeader.into()),
                None => return Err(MissingAuthentication.into()),
            };

            let token = crate::token::parse_token(&auth_header[7..], &settings.jwt_secret)?;

            let id = token.user();
            // TODO: Maybe handle case where user ID does not exist as a special case although in
            // practise this is probably just an INTERNAL_SERVER_ERROR
            let user = tokio::task::spawn_blocking(move || {
                let conn = pool.get().unwrap();
                doxa_db::action::user::get_user_by_id(&conn, id)
            })
            .await?
            .map_err(|e| {
                if was_unique_key_violation(&e) {
                    UserNotFound.into()
                } else {
                    RespondableErrorWrapper::from(e)
                }
            })?;

            if token.generation() != user.token_generation {
                return Err(IncorrectTokenGeneration.into());
            }

            Ok(AuthGuard::new(token.user(), user.admin, ()))
        })
    }
}
