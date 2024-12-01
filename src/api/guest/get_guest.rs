use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    security::WithClaims,
    validation::{Validate, Validator},
};

use super::GuestIdCard;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GetGuestInput {
    pub guest_id: Uuid,
}
impl Validate for GetGuestInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        Ok(())
    }
}
impl WithClaims for GetGuestInput {
    fn with_claims(self, _claims: crate::security::Claims) -> Self {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GetGuestOutput {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Date,
    pub id_card: Option<GuestIdCard>,
    pub phone_number: Option<String>,
}
