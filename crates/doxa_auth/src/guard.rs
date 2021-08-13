use std::{future::Future, pin::Pin};

use actix_web::{web, HttpResponse};

pub struct AuthGuard<T: AuthGuardInner> {
    user: i32,
    inner: T,
}

impl<T: AuthGuardInner> AuthGuard<T> {
    pub fn new(user: i32, inner: T) -> Self {
        AuthGuard { user, inner }
    }

    pub fn user(&self) -> i32 {
        self.user
    }

    pub fn inner(self) -> T {
        self.inner
    }
}

// TODO: think of better name
pub trait AuthGuardInner: Sized {
    fn construct(
        user: i32,
        connnection: web::Data<doxa_db::PgPool>,
    ) -> Pin<Box<dyn Future<Output = Result<Self, HttpResponse>>>>;
}

impl AuthGuardInner for () {
    fn construct(
        _user: i32,
        _connnection: web::Data<doxa_db::PgPool>,
    ) -> Pin<Box<dyn Future<Output = Result<Self, HttpResponse>>>> {
        Box::pin(async { Ok(()) })
    }
}
