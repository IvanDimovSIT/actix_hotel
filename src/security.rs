use std::error::Error;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::sea_query::token;
use serde::{Deserialize, Serialize};
use sha256::digest;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{app_state::AppState, constants::SALT_LENGTH, persistence::user::Role};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub user_id: Uuid,
    pub role: Role,
    pub exp: u64,
}
impl Claims {
    pub fn from_token(token: &str, app_state: &AppState) -> Result<Self, Box<dyn Error>> {
        let secret = &app_state.security_info.jwt_secret;
        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        );

        if let Ok(header) = result {
            Ok(header.claims)
        } else {
            Err(Box::new(result.err().unwrap()))
        }
    }

    pub fn to_token(&self, app_state: &AppState) -> Result<String, Box<dyn Error>> {
        let secret = &app_state.security_info.jwt_secret;
        let result = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        );

        if let Ok(token) = result {
            Ok(token)
        } else {
            Err(Box::new(result.err().unwrap()))
        }
    }
}

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
