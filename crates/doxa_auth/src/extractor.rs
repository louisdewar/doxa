use doxa_core::{
    autha_client::{error::AuthaError, jwt::Scope},
    error::RespondableErrorWrapper,
};

use std::future::Future;
use std::pin::Pin;

use actix_web::{dev, web, FromRequest, HttpRequest};

use crate::{
    error::{InvalidAuthenticationHeader, MissingAuthentication, NotAccessToken},
    guard::AuthGuard,
    settings::Settings,
};

impl FromRequest for AuthGuard<()> {
    type Error = RespondableErrorWrapper;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
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

            let token = &auth_header[7..];

            if token == settings.system_account_secret {
                return Ok(AuthGuard::new(None, true, ()));
            }

            let token = settings
                .autha_client
                .verify_jwt(token)
                .map_err(AuthaError::from)?;

            if !token.has_scope(&Scope::Access) {
                return Err(NotAccessToken.into());
            }

            let admin = token.has_scope(&Scope::Admin);

            Ok(AuthGuard::new(Some(token.user()), admin, ()))
        })
    }
}
