use actix_web::http::StatusCode;
use sea_orm::{prelude::Date, sqlx::types::chrono::Utc};
use serde::{Deserialize, Serialize};
use utoipa::{schema, ToSchema};

use crate::validation::{Validate, Validator};

use super::error_response::ErrorResponse;

pub mod add_guest;

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
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse> {
        validator.validate_ucn(&self.ucn)?;
        validator.validate_id_card_number(&self.id_card_number)?;
        validator.validate_id_card_issue_authority(&self.issue_authority)?;

        if self.issue_date > self.validity {
            return Err(ErrorResponse::new(
                "Issue date must be before validity".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        } else if self.validity < Utc::now().date_naive() {
            return Err(ErrorResponse::new(
                "Card is expired".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        Ok(())
    }
}
