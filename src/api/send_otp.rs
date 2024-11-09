use actix_web::{body::BoxBody, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::validation::{Validate, Validator};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SendOtpInput {
    #[schema(example = "user@example.com", required = true)]
    pub email: String,
}
impl Validate for SendOtpInput {
    fn validate(&self, validator: &Validator) -> Result<(), HttpResponse<BoxBody>> {
        validator.validate_email(&self.email)?;

        return Ok(());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SendOtpOutput;
