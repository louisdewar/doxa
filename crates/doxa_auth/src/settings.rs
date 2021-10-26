use hmac::{Hmac, NewMac};
use sha2::Sha256;

#[derive(Clone)]
pub struct Settings {
    pub jwt_secret: Hmac<Sha256>,
    pub allow_registration: bool,
}

pub fn generate_jwt_hmac(secret: &[u8]) -> Hmac<Sha256> {
    Hmac::new_from_slice(secret).unwrap()
}

pub fn generate_rand_jwt_secret() -> Vec<u8> {
    use rand::Rng;
    rand::thread_rng()
        .sample_iter(rand::distributions::Standard)
        .take(20)
        .collect()
}
