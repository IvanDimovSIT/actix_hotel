use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    api::error_response::ErrorResponse,
    security::WithClaims,
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct PromoteInput {
    #[schema(example = "user@example.com", required = true)]
    pub email: String,
}
impl Validate for PromoteInput {
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse> {
        validator.validate_email(&self.email)?;

        return Ok(());
    }
}
impl WithClaims for PromoteInput {
    fn with_claims(self, _claims: crate::security::Claims) -> Self {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct PromoteOutput;
