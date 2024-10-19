use rand::{distributions::Alphanumeric, Rng};
use sha256::digest;

use crate::constants::SALT_LENGTH;

pub fn hash_with_salt(password: &str, salt: &str) -> String {
    digest(password.to_string() + salt)
}

pub fn passwords_match(raw_password: &str, salt: &str, password_hash: &str) -> bool {
    hash_with_salt(raw_password, salt) == password_hash
}

pub fn generate_salt() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SALT_LENGTH)
        .map(char::from)
        .collect()
}