use ::serde::{Deserialize, Serialize};
use actix_web::{body::BoxBody, HttpResponse};
use utoipa::{schema, ToSchema};

use crate::{
    api::error_response::ErrorResponse,
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct ResetPasswordInput {
    #[schema(example = "user@example.com", required = true)]
    pub email: String,
    #[schema(example = "Abcd1234", required = true)]
    pub otp: String,
    #[schema(example = "12345678", required = true)]
    pub new_password: String,
}
impl Validate for ResetPasswordInput {
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse> {
        validator.validate_email(&self.email)?;
        validator.validate_password(&self.new_password)?;
        validator.validate_otp(&self.otp)?;

        return Ok(());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct ResetPasswordOutput;
