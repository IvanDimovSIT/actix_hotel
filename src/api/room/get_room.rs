use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    persistence::room::BathroomType,
    security::WithClaims,
    validation::{Validate, Validator},
};

use super::Bed;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GetRoomInput {
    pub room_id: Uuid,
}
impl Validate for GetRoomInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        Ok(())
    }
}
impl WithClaims for GetRoomInput {
    fn with_claims(self, _claims: crate::security::Claims) -> Self {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GetRoomOutput {
    pub id: Uuid,
    pub price: i64,
    pub floor: i16,
    pub room_number: String,
    pub bathroom_type: BathroomType,
    pub beds: Vec<Bed>,
}
