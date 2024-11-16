use actix_web::{body::BoxBody, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    api::error_response::ErrorResponse,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct PromoteOutput;
