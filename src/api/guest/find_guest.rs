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


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct FindGuestInput {
    #[schema(example = "John", required = false)]
    pub first_name: Option<String>,

    #[schema(example = "Smith", required = false)]
    pub last_name: Option<String>,

    #[schema(example = "1990-01-01", required = false)]
    pub date_of_birth: Option<Date>,

    #[schema(example = "0123456789", required = false)]
    pub ucn: Option<String>,

    #[schema(example = "+359123456789", required = false)]
    pub phone_number: Option<String>,
}
impl Validate for FindGuestInput {
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse> {
        if let Some(first_name) = &self.first_name {
            validator.validate_name(first_name)?;
        }
        if let Some(last_name) = &self.last_name {
            validator.validate_name(last_name)?;
        }
        if let Some(phone_number) = &self.phone_number {
            validator.validate_phone_number(phone_number)?;
        }
        if let Some(ucn) = &self.ucn {
            validator.validate_ucn(ucn)?;
        }
        if let Some(date_of_birth) = &self.date_of_birth {
            if *date_of_birth >= Utc::now().date_naive() {
                return Err(ErrorResponse::new(
                    "Date of birth needs to be a past date".to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }
        }

        Ok(())
    }
}
impl WithClaims for FindGuestInput {
    fn with_claims(self, _claims: crate::security::Claims) -> Self {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct FindGuestOutput {
    pub guest_ids: Vec<Uuid>,
}
