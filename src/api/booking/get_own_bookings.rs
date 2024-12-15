use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    security::WithClaims,
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GetOwnBookingsInput {
    pub user_id: Option<Uuid>,
    pub include_canceled: bool,
    pub include_paid: bool,
}
impl Validate for GetOwnBookingsInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        Ok(())
    }
}
impl WithClaims for GetOwnBookingsInput {
    fn with_claims(self, claims: crate::security::Claims) -> Self {
        Self {
            user_id: Some(claims.user_id),
            ..self
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GetOwnBookingsOutput {
    pub ids: Vec<Uuid>,
}
