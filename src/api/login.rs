use actix_web::{body::BoxBody, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{openapi::schema, ToSchema};

use crate::validation::{Validate, Validator};

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
    fn validate(&self, validator: &Validator) -> Result<(), HttpResponse<BoxBody>> {
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
