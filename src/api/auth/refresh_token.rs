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
pub struct RefreshTokenInput {
    pub claims: Option<Claims>,
}
impl Validate for RefreshTokenInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        Validator::validate_option(&self.claims, "claims")?;

        Ok(())
    }
}
impl WithClaims for RefreshTokenInput {
    fn with_claims(self, claims: Claims) -> Self {
        Self {
            claims: Some(claims),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct RefreshTokenOutput {
    pub token: String,
}
