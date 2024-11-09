use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use regex::Regex;

use crate::services::error_response;

pub trait Validate {
    fn validate(&self, validator: &Validator) -> Result<(), HttpResponse<BoxBody>>;
}

#[derive(Clone)]
pub struct Validator {
    email_regex: Regex,
    password_regex: Regex,
    room_number_regex: Regex,
}
impl Validator {
    pub fn new() -> Self {
        Self {
            email_regex: Regex::new("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")
                .expect("Error creating email regex"),
            password_regex: Regex::new("^[a-zA-Z0-9!@#$%^&*(){}]{8,20}$")
                .expect("Error creating password regex"),
            room_number_regex: Regex::new("^[0-9]{1,5}[A-Z]?$")
                .expect("Error creating room number regex"),
        }
    }

    pub fn validate_email(&self, email: &str) -> Result<(), HttpResponse<BoxBody>> {
        if self.email_regex.is_match(email) {
            return Ok(());
        }

        Err(error_response(
            format!("Invalid email: {}", email),
            StatusCode::BAD_REQUEST,
        ))
    }

    pub fn validate_password(&self, password: &str) -> Result<(), HttpResponse<BoxBody>> {
        if self.password_regex.is_match(password) {
            return Ok(());
        }

        Err(error_response(
                "Invalid password: Needs to be between 8 and 20 characters (letters, numbers and symbols)".to_string(),
                StatusCode::BAD_REQUEST
            )
        )
    }

    pub fn validate_room_number(&self, room_number: &str) -> Result<(), HttpResponse<BoxBody>> {
        if self.room_number_regex.is_match(room_number) {
            return Ok(());
        }
        Err(error_response(
            "Invalid room number: Needs to be numbers optionally followed by an upper case letter"
                .to_string(),
            StatusCode::BAD_REQUEST,
        ))
    }

    pub fn validate_option<T>(
        &self,
        option: &Option<T>,
        field_name: &str,
    ) -> Result<(), HttpResponse<BoxBody>> {
        if option.is_some() {
            return Ok(());
        }

        Err(error_response(
            format!("No input for '{}'", field_name),
            StatusCode::BAD_REQUEST,
        ))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        let validator = Validator::new();
        let valid_email = "myemail123@example.com";
        let invalid_email = "invalidemail@";
        assert!(validator.validate_email(valid_email).is_ok());
        assert!(validator.validate_email(invalid_email).is_err());
    }

    #[test]
    fn test_validate_password() {
        let validator = Validator::new();
        let valid_password = "MyPassword";
        let invalid_password = "abc";
        assert!(validator.validate_password(valid_password).is_ok());
        assert!(validator.validate_password(invalid_password).is_err());
    }

    #[test]
    fn test_validate_room_number() {
        let validator = Validator::new();
        let valid_room_number = "132B";
        let invalid_room_number = "number";
        assert!(validator.validate_room_number(valid_room_number).is_ok());
        assert!(validator.validate_room_number(invalid_room_number).is_err());
    }

    #[test]
    fn test_validate_option() {
        let validator = Validator::new();
        let valid_option = Some("some");
        let invalid_option: Option<&str> = None;
        assert!(validator.validate_option(&valid_option, "option").is_ok());
        assert!(validator
            .validate_option(&invalid_option, "option")
            .is_err());
    }
}
