use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::{prelude::Date, sqlx::types::chrono::Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    services::error_response,
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GuestIdCard {
    #[schema(example = "0123456789", required = false)]
    pub ucn: String,

    #[schema(example = "012345678", required = false)]
    pub id_card_number: String,

    #[schema(example = "Sofia", required = false)]
    pub issue_authority: String,

    #[schema(example = "2010-01-01", required = false)]
    pub issue_date: Date,

    #[schema(example = "2020-01-01", required = false)]
    pub validity: Date,
}
impl Validate for GuestIdCard {
    fn validate(&self, validator: &Validator) -> Result<(), HttpResponse<BoxBody>> {
        validator.validate_ucn(&self.ucn)?;
        validator.validate_id_card_number(&self.id_card_number)?;
        validator.validate_id_card_issue_authority(&self.issue_authority)?;

        if self.issue_date > self.validity {
            return Err(error_response(
                "Issue date must be before validity".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        } else if self.validity < Utc::now().date_naive() {
            return Err(error_response(
                "Card is expired".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct AddGuestInput {
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
impl Validate for AddGuestInput {
    fn validate(&self, validator: &Validator) -> Result<(), HttpResponse<BoxBody>> {
        validator.validate_name(&self.first_name)?;
        validator.validate_name(&self.last_name)?;

        if self.date_of_birth >= Utc::now().date_naive() {
            return Err(error_response(
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct AddGuestOutput {
    pub guest_id: Uuid,
}
