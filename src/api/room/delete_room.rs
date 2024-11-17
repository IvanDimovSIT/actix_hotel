use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    security::{Claims, WithClaims},
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct DeleteRoomInput {
    pub room_id: Uuid,
}
impl WithClaims for DeleteRoomInput {
    fn with_claims(self, _claims: Claims) -> Self {
        self
    }
}
impl Validate for DeleteRoomInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct DeleteRoomOutput;
