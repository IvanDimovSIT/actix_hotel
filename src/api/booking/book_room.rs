use std::collections::HashSet;

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
pub struct BookRoomInput {
    #[serde(skip)]
    pub booked_by: Option<Uuid>,

    pub room_id: Uuid,

    #[schema(example = "2025-01-01", required = true)]
    pub start_date: Date,

    #[schema(example = "2025-01-05", required = true)]
    pub end_date: Date,

    pub main_guest: Uuid,

    pub other_guests: HashSet<Uuid>,

    pub guest_user_id: Option<Uuid>,
}
impl Validate for BookRoomInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        if self.start_date < Utc::now().date_naive() {
            return Err(ErrorResponse::new(
                "Start date cannot be a past date".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        if self.end_date < Utc::now().date_naive() {
            return Err(ErrorResponse::new(
                "End date cannot be a past date".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        if self.start_date > self.end_date {
            return Err(ErrorResponse::new(
                "Start date cannot be after end date".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        if self.other_guests.len() > 10 {
            return Err(ErrorResponse::new(
                "Too many guests".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        Validator::validate_option(&self.booked_by, "booked_by")?;

        Ok(())
    }
}
impl WithClaims for BookRoomInput {
    fn with_claims(self, claims: crate::security::Claims) -> Self {
        Self {
            booked_by: Some(claims.user_id),
            ..self
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct BookRoomOutput {
    pub booking_id: Uuid,
}
