pub struct AuthGuard {
    user: i32,
}

impl AuthGuard {
    pub fn user(&self) -> i32 {
        self.user
    }
}
