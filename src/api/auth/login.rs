use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    api::error_response::ErrorResponse,
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct LoginInput {
    #[schema(example = "user@example.com", required = true)]
    pub email: String,
    #[schema(example = "12345678", required = true)]
    pub password: String,
}
impl Validate for LoginInput {
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse> {
        validator.validate_email(&self.email)?;

        validator.validate_password(&self.password)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct LoginOutput {
    pub token: String,
}
