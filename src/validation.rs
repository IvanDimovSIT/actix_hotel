use actix_web::http::StatusCode;
use regex::Regex;

use crate::{api::error_response::ErrorResponse, constants::OTP_LENGTH};

pub trait Validate {
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse>;
}

#[derive(Clone)]
pub struct Validator {
    email_regex: Regex,
    password_regex: Regex,
    room_number_regex: Regex,
    otp_regex: Regex,
    name_regex: Regex,
    ucn_regex: Regex,
    id_card_number_regex: Regex,
    phone_number_regex: Regex,
    id_card_issue_authority_regex: Regex,
    comment_contents_regex: Regex,
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
            otp_regex: Regex::new("^[a-zA-Z0-9]+$").expect("Error creating otp regex"),
            name_regex: Regex::new("^[A-Za-z-']{2,32}$").expect("Error creating name regex"),
            ucn_regex: Regex::new("^[0-9]{10}$").expect("Error creating ucn regex"),
            id_card_number_regex: Regex::new("^[0-9]{9}$")
                .expect("Error creating id card number regex"),
            phone_number_regex: Regex::new(r"^\+[0-9]{8,15}$")
                .expect("Error creating phone number regex"),
            id_card_issue_authority_regex: Regex::new("^[A-Za-z]+(?: [A-Za-z]+)*$")
                .expect("Error creating id card issue authority regex"),
            comment_contents_regex: Regex::new("^(\\s*[^ \\t\\r\\n].{0,255})$")
                .expect("Error creating comment contents regex"),
        }
    }

    fn validate<F>(regex: &Regex, field: &str, message_provider: F) -> Result<(), ErrorResponse>
    where
        F: Fn() -> String,
    {
        if regex.is_match(field) {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                message_provider(),
                StatusCode::BAD_REQUEST,
            ))
        }
    }

    pub fn validate_email(&self, email: &str) -> Result<(), ErrorResponse> {
        Self::validate(&self.email_regex, email, || {
            format!("Invalid email: {}", email)
        })
    }

    pub fn validate_password(&self, password: &str) -> Result<(), ErrorResponse> {
        Self::validate(&self.password_regex, password, || {
            "Invalid password: Needs to be between 8 and 20 characters (letters, numbers and symbols)".to_string()
        })
    }

    pub fn validate_room_number(&self, room_number: &str) -> Result<(), ErrorResponse> {
        Self::validate(&self.room_number_regex, room_number, || {
            "Invalid room number: Needs to be numbers optionally followed by an upper case letter"
                .to_string()
        })
    }

    pub fn validate_otp(&self, otp: &str) -> Result<(), ErrorResponse> {
        if otp.len() != OTP_LENGTH {
            return Err(ErrorResponse::new(
                format!("Invalid otp: Needs to be {OTP_LENGTH} characters long"),
                StatusCode::BAD_REQUEST,
            ));
        }
        Self::validate(&self.otp_regex, otp, || {
            "Invalid otp: Needs to be contain only alphanumeric characters".to_string()
        })
    }

    pub fn validate_name(&self, name: &str) -> Result<(), ErrorResponse> {
        Self::validate(&self.name_regex, name, || {
            format!("Invalid name '{}'", name)
        })
    }

    pub fn validate_ucn(&self, ucn: &str) -> Result<(), ErrorResponse> {
        Self::validate(&self.ucn_regex, ucn, || format!("Invalid ucn '{}'", ucn))
    }

    pub fn validate_id_card_issue_authority(
        &self,
        id_card_issue_authority: &str,
    ) -> Result<(), ErrorResponse> {
        Self::validate(
            &self.id_card_issue_authority_regex,
            id_card_issue_authority,
            || format!("Invalid id issue authority '{}'", id_card_issue_authority),
        )
    }

    pub fn validate_id_card_number(&self, id_card_number: &str) -> Result<(), ErrorResponse> {
        Self::validate(&self.id_card_number_regex, id_card_number, || {
            format!("Invalid id card number '{}'", id_card_number)
        })
    }

    pub fn validate_phone_number(&self, phone_number: &str) -> Result<(), ErrorResponse> {
        Self::validate(&self.phone_number_regex, phone_number, || {
            format!("Invalid phone number '{}'", phone_number)
        })
    }

    pub fn validate_comment_contents(&self, comment_contents: &str) -> Result<(), ErrorResponse> {
        Self::validate(&self.comment_contents_regex, comment_contents, || {
            format!("Invalid comment contents '{}'", comment_contents)
        })
    }

    pub fn validate_option<T>(option: &Option<T>, field_name: &str) -> Result<(), ErrorResponse> {
        if option.is_some() {
            return Ok(());
        }

        Err(ErrorResponse::new(
            format!("No input for '{}'", field_name),
            StatusCode::BAD_REQUEST,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::security::generate_otp;

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
    fn test_validate_ucn() {
        let validator = Validator::new();
        let valid_ucn = "0123456789";
        let invalid_ucn = "05689";
        assert!(validator.validate_ucn(valid_ucn).is_ok());
        assert!(validator.validate_ucn(invalid_ucn).is_err());
    }

    #[test]
    fn test_validate_id_card_number() {
        let validator = Validator::new();
        let valid_id_card = "012345678";
        let invalid_id_card = "a1234567";
        assert!(validator.validate_id_card_number(valid_id_card).is_ok());
        assert!(validator.validate_id_card_number(invalid_id_card).is_err());
    }

    #[test]
    fn test_validate_phone_number() {
        let validator = Validator::new();
        let valid_phone_number = "+359123456789";
        let invalid_phone_number = "0123456789";
        assert!(validator.validate_phone_number(valid_phone_number).is_ok());
        assert!(validator
            .validate_phone_number(invalid_phone_number)
            .is_err());
    }

    #[test]
    fn test_validate_comment_contents() {
        let validator = Validator::new();
        let valid_contents = " some example text";
        let invalid_contents = "   \t  \t \n\t ";
        assert!(validator.validate_comment_contents(valid_contents).is_ok());
        assert!(validator
            .validate_comment_contents(invalid_contents)
            .is_err());
    }

    #[test]
    fn test_validate_otp() {
        let validator = Validator::new();

        for _ in 0..30 {
            let valid_otp = generate_otp();
            assert!(validator.validate_otp(&valid_otp).is_ok());
        }

        let invalid_otp = " invalid";
        assert!(validator.validate_otp(invalid_otp).is_err());
    }

    #[test]
    fn test_validate_option() {
        let valid_option = Some("some");
        let invalid_option: Option<&str> = None;
        assert!(Validator::validate_option(&valid_option, "option").is_ok());
        assert!(Validator::validate_option(&invalid_option, "option").is_err());
    }
}
