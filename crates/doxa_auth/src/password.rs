// TODO: spend time considering constants
const SALT_BYTES_LEN: usize = 16;
const ROUNDS: u32 = 50;
const HASH_SIZE_BYTES: usize = 32;

const MAX_PASSWORD_LEN: usize = 256;

pub fn new_hashed(password: &str) -> String {
    use rand::Rng;

    let salt: Vec<u8> = rand::thread_rng()
        .sample_iter(rand::distributions::Standard)
        .take(SALT_BYTES_LEN)
        .collect();

    let mut output = vec![0; HASH_SIZE_BYTES];
    bcrypt_pbkdf::bcrypt_pbkdf(password, &salt, ROUNDS, &mut output).expect("bcrypt failed!");

    format!("{} {}", base64::encode(salt), base64::encode(output))
}

pub fn verify(password: &str, hashed_password: &str) -> bool {
    let parts: Vec<&str> = hashed_password.split(' ').collect();

    assert_eq!(
        parts.len(),
        2,
        "Invalid formatting of passwords in database"
    );

    let salt = parts[0];
    let hashed_password = parts[1];
    let salt = base64::decode(salt).expect("Salt was not base64");
    let hashed_password = base64::decode(hashed_password).expect("Hashed password was not base64");

    let mut output = vec![0; HASH_SIZE_BYTES];
    bcrypt_pbkdf::bcrypt_pbkdf(password, &salt, ROUNDS, &mut output).expect("bcrypt failed!");
    output == hashed_password
}

/// Whether a password satisfies minimum constraints on complexity
pub fn is_allowed(password: &str) -> bool {
    password.len() > 5 && password.len() <= MAX_PASSWORD_LEN
}
