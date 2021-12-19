use std::{future::Future, pin::Pin};

use actix_web::{web, HttpResponse};
use doxa_core::RespondableError;

use crate::error::UserNotAdmin;

pub struct AuthGuard<T: AuthGuardInner = ()> {
    user: i32,
    is_admin: bool,
    inner: T,
}

impl<T: AuthGuardInner> AuthGuard<T> {
    pub fn new(user: i32, is_admin: bool, inner: T) -> Self {
        AuthGuard {
            user,
            is_admin,
            inner,
        }
    }

    pub fn id(&self) -> i32 {
        self.user
    }

    pub fn admin(&self) -> bool {
        self.is_admin
    }

    pub fn inner(self) -> T {
        self.inner
    }
}

// TODO: think of better name
pub trait AuthGuardInner: Sized {
    fn construct(
        user: i32,
        is_admin: bool,
        connnection: web::Data<doxa_db::PgPool>,
    ) -> Pin<Box<dyn Future<Output = Result<Self, HttpResponse>>>>;
}

impl AuthGuardInner for () {
    fn construct(
        _user: i32,
        _is_admin: bool,
        _connnection: web::Data<doxa_db::PgPool>,
    ) -> Pin<Box<dyn Future<Output = Result<Self, HttpResponse>>>> {
        Box::pin(async { Ok(()) })
    }
}

/// Guard that requires the user to be an admin
pub struct Admin;

impl AuthGuardInner for Admin {
    fn construct(
        _user: i32,
        is_admin: bool,
        _connnection: web::Data<doxa_db::PgPool>,
    ) -> Pin<Box<dyn Future<Output = Result<Self, HttpResponse>>>> {
        Box::pin(async move {
            if is_admin {
                Ok(Admin)
            } else {
                Err(UserNotAdmin.as_response())
            }
        })
    }
}
