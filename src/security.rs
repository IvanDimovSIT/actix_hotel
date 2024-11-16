use std::error::Error;

use actix_web::{body::BoxBody, http::StatusCode, HttpRequest, HttpResponse};
use bcrypt::{hash, verify};
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    app_state::AppState,
    constants::{BCRYPT_COST, BEARER_PREFIX, OTP_LENGTH},
    persistence::user::Role,
    util::error_to_response,
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

pub fn hash_password(password: &str) -> String {
    hash(password, BCRYPT_COST).expect("Error hashing password")
}

pub fn passwords_match(raw_password: &str, password_hash: &str) -> bool {
    verify(raw_password, password_hash).expect("Error verifying password")
}

pub fn generate_otp() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(OTP_LENGTH)
        .map(char::from)
        .collect()
}

pub fn decode_claims(
    req: &HttpRequest,
    app_state: &AppState,
    roles: &[Role],
) -> Result<Claims, ErrorResponse> {
    let auth_header_option = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if auth_header_option.is_none() {
        return Err(ErrorResponse::new(
            "Not authenticated: missing JWT".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }
    let auth_header = auth_header_option.unwrap();
    if !auth_header.starts_with(BEARER_PREFIX) {
        return Err(ErrorResponse::new(
            "Not authenticated: invalid JWT format".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let decoded = Claims::from_token(auth_header.strip_prefix(BEARER_PREFIX).unwrap(), app_state);
    if let Err(err) = decoded {
        return Err(error_to_response(err));
    }

    let claims = decoded.unwrap();
    if claims.exp < get_current_timestamp() {
        return Err(ErrorResponse::new(
            "Not authenticated: expired JWT".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let has_role = roles.iter().any(|role| *role == claims.role);

    if has_role {
        Ok(claims)
    } else {
        Err(ErrorResponse::new(
            "Insufficient access".to_string(),
            StatusCode::FORBIDDEN,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_match() {
        let raw_password = "mypassword1".to_string();
        let hashed_password = hash_password(&raw_password);

        assert!(passwords_match(&raw_password, &hashed_password));
    }

    #[test]
    fn test_password_hashing_wrong_password() {
        let raw_password = "mypassword1".to_string();
        let invalid_password = "Mypassword1".to_string();
        let hashed_password = hash_password(&raw_password);

        assert!(!passwords_match(&invalid_password, &hashed_password));
    }

    #[test]
    fn test_password_hashing_empty() {
        let raw_password = "".to_string();
        let hashed_password = hash_password(&raw_password);

        assert!(passwords_match(&raw_password, &hashed_password));
    }

    #[test]
    fn test_password_hashing_long() {
        let raw_password = "a".repeat(256).to_string();
        let hashed_password = hash_password(&raw_password);

        assert!(passwords_match(&raw_password, &hashed_password));
    }
}
