use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    api::error_response::ErrorResponse,
    security::Claims,
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct RefreshTokenInput {
    pub claims: Claims,
}
impl Validate for RefreshTokenInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct RefreshTokenOutput {
    pub token: String,
}
