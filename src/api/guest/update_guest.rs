use actix_web::http::StatusCode;
use sea_orm::{prelude::Date, sqlx::types::chrono::Utc};
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
pub struct UpdateGuestInput {
    #[serde(skip)]
    pub id: Option<Uuid>,

    #[schema(example = "John", required = true)]
    pub first_name: String,

    #[schema(example = "Johnson", required = true)]
    pub last_name: String,

    #[schema(example = "1990-01-01", required = true)]
    pub date_of_birth: Date,

    #[schema(required = false)]
    pub id_card: Option<GuestIdCard>,

    #[schema(example = "+359123456789", required = false)]
    pub phone_number: Option<String>,
}
impl Validate for UpdateGuestInput {
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse> {
        validator.validate_name(&self.first_name)?;
        validator.validate_name(&self.last_name)?;

        Validator::validate_option(&self.id, "id")?;

        if self.date_of_birth >= Utc::now().date_naive() {
            return Err(ErrorResponse::new(
                "Date of birth needs to be a past date".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }
        if let Some(card) = &self.id_card {
            card.validate(validator)?;
        }

        if let Some(phone_number) = &self.phone_number {
            validator.validate_phone_number(&phone_number)?;
        }

        Ok(())
    }
}
impl WithClaims for UpdateGuestInput {
    fn with_claims(self, _claims: crate::security::Claims) -> Self {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct UpdateGuestOutput;
