use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    api::error_response::ErrorResponse,
    security::{Claims, WithClaims},
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct LogoutInput {
    #[serde(skip)]
    pub claims: Option<Claims>,
}
impl Validate for LogoutInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        Validator::validate_option(&self.claims, "JWT")?;
        Ok(())
    }
}
impl WithClaims for LogoutInput {
    fn with_claims(self, claims: Claims) -> Self {
        Self {
            claims: Some(claims),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct LogoutOutput;
