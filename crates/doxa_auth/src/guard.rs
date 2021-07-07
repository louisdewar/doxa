use std::future::Future;

use actix_web::HttpResponse;
use diesel::PgConnection;

pub struct AuthGuard<T: AuthGuardPart> {
    user: i32,
    inner: T,
}

impl<T: AuthGuardPart> AuthGuard<T> {
    pub fn user(&self) -> i32 {
        self.user
    }

    pub fn inner(self) -> T {
        self.inner
    }
}

// TODO: think of better name
pub trait AuthGuardPart: Sized {
    fn construct(
        user: i32,
        connnection: PgConnection,
    ) -> Box<dyn Future<Output = Result<Self, HttpResponse>>>;
}
