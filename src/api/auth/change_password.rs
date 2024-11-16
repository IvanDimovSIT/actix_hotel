use actix_web::{body::BoxBody, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    security::WithClaims,
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct ChangePasswordInput {
    #[serde(skip)]
    pub user_id: Uuid,
    #[schema(example = "12345678", required = true)]
    pub old_password: String,
    #[schema(example = "12345678", required = true)]
    pub new_password: String,
}
impl Validate for ChangePasswordInput {
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse> {
        validator.validate_password(&self.old_password)?;
        validator.validate_password(&self.new_password)?;

        Ok(())
    }
}
impl WithClaims for ChangePasswordInput {
    fn with_claims(self, claims: crate::security::Claims) -> Self {
        Self {
            user_id: claims.user_id,
            ..self
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct ChangePasswordOutput;
