use actix_web::{body::BoxBody, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::validation::{Validate, Validator};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
impl Validate for LoginInput {
    fn validate(&self, validator: &Validator) -> Result<(), HttpResponse<BoxBody>> {
        if let Err(err) = validator.validate_email(&self.email) {
            return Err(err);
        }

        if let Err(err) = validator.validate_password(&self.password) {
            return Err(err);
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginOutput {
    pub token: String,
}
