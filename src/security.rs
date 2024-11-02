use std::error::Error;

use actix_web::{body::BoxBody, HttpRequest, HttpResponse};
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::sea_query::token;
use serde::{Deserialize, Serialize};
use sha256::digest;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    app_state::{self, AppState},
    constants::{BEARER_PREFIX, SALT_LENGTH},
    persistence::user::Role,
};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
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

pub fn decode_claims(
    req: &HttpRequest,
    app_state: &AppState,
    roles: &[Role],
) -> Result<Claims, HttpResponse<BoxBody>> {
    let auth_header_option = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if auth_header_option.is_none() {
        return Err(HttpResponse::Unauthorized().body("Not authenticated: missing JWT"));
    }
    let auth_header = auth_header_option.unwrap();
    if !auth_header.starts_with(BEARER_PREFIX) {
        return Err(HttpResponse::Unauthorized().body("Not authenticated: invalid JWT format"));
    }

    let decoded = Claims::from_token(auth_header.strip_prefix(BEARER_PREFIX).unwrap(), app_state);
    if let Err(err) = decoded {
        return Err(HttpResponse::from_error(err));
    }

    let claims = decoded.unwrap();
    if claims.exp < get_current_timestamp() {
        return Err(HttpResponse::Unauthorized().body("Not authenticated: expired JWT"));
    }

    let has_role = roles.iter().any(|role| *role == claims.role);

    if has_role {
        Ok(claims)
    } else {
        Err(HttpResponse::Forbidden().body("Insufficient access"))
    }
}
