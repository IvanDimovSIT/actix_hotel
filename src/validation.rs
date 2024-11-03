use actix_web::{body::BoxBody, HttpResponse};
use regex::Regex;

use crate::util::require_some;

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

        Err(HttpResponse::BadRequest().body(format!("Invalid email: {}", email)))
    }

    pub fn validate_password(&self, password: &str) -> Result<(), HttpResponse<BoxBody>> {
        if self.password_regex.is_match(password) {
            return Ok(());
        }

        Err(HttpResponse::BadRequest().body("Invalid password: Needs to be between 8 and 20 characters (letters, numbers and symbols)"))
    }

    pub fn validate_room_number(&self, room_number: &str) -> Result<(), HttpResponse<BoxBody>> {
        if self.room_number_regex.is_match(room_number) {
            return Ok(());
        }

        Err(HttpResponse::BadRequest().body(
            "Invalid room number: Needs to be numbers optionally followed by an upper case letter",
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

        Err(HttpResponse::BadRequest().body(format!("No input for '{}'", field_name)))
    }
}
