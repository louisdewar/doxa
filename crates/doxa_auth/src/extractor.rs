use doxa_core::{handle_doxa_error, RespondableError};
use doxa_db::PgPool;

use std::future::Future;
use std::pin::Pin;

use actix_web::{dev, web, FromRequest, HttpRequest, HttpResponse};

use crate::{
    error::{InvalidAuthenticationHeader, MissingAuthentication},
    guard::{AuthGuard, AuthGuardInner},
    settings::Settings,
};

impl<T: AuthGuardInner> FromRequest for AuthGuard<T> {
    type Error = HttpResponse;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

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
                        return Err(InvalidAuthenticationHeader.into_response());
                    }
                    val
                }
                Some(Err(_)) => return Err(InvalidAuthenticationHeader.into_response()),
                None => return Err(MissingAuthentication.into_response()),
            };

            let token = handle_doxa_error!(crate::token::parse_token(
                &auth_header,
                &settings.jwt_secret
            ));

            // In future there will be support for disabling
            // sessions and the check will be done here (as part of a DB call)

            let inner = T::construct(token.user(), pool).await?;
            Ok(AuthGuard::new(token.user(), inner))
        })
    }
}
