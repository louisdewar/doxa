use doxa_core::error::RespondableErrorWrapper;

use std::future::Future;
use std::pin::Pin;

use actix_web::{dev, web, FromRequest, HttpRequest};

use crate::{
    error::{InvalidAuthenticationHeader, MissingAuthentication},
    guard::AuthGuard,
    settings::Settings,
};

impl FromRequest for AuthGuard<()> {
    type Error = RespondableErrorWrapper;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        // TODO: use database to check to see if user still exists.
        // also maybe use an EPOCH scheme to increment a number if a user requests sign outs on all
        // devices.
        // let pool = req.app_data::<web::Data<PgPool>>().unwrap().clone();
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

            // In future there will be support for disabling
            // sessions and the check will be done here (as part of a DB call)

            // let inner = T::construct(token.user(), pool).await?;
            Ok(AuthGuard::new(token.user(), ()))
        })
    }
}
