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

const MAX_CAPACITY: i16 = 20;
const MIN_CAPACITY: i16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct FindUnoccupiedRoomsInput {
    #[schema(example = "2025-01-01", required = true)]
    pub start_date: Date,

    #[schema(example = "2025-01-05", required = true)]
    pub end_date: Date,

    #[schema(required = false)]
    pub minimum_capacity: Option<i16>,

    #[schema(required = false)]
    pub maximum_capacity: Option<i16>,
}
impl Validate for FindUnoccupiedRoomsInput {
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

        if let Some(max) = self.maximum_capacity {
            if max > MAX_CAPACITY {
                return Err(ErrorResponse::new(
                    format!("Maximum capacity: {MAX_CAPACITY}").to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            if max < MIN_CAPACITY {
                return Err(ErrorResponse::new(
                    format!("Minimum capacity: {MIN_CAPACITY}").to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }
        }

        if let Some(min) = self.minimum_capacity {
            if min > MAX_CAPACITY {
                return Err(ErrorResponse::new(
                    format!("Maximum capacity: {MAX_CAPACITY}").to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            if min < MIN_CAPACITY {
                return Err(ErrorResponse::new(
                    format!("Minimum capacity: {MIN_CAPACITY}").to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            if self.maximum_capacity.is_some() && self.maximum_capacity.unwrap() < min {
                return Err(ErrorResponse::new(
                    format!("Maximum capacity cannot be less than minimum capacity").to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }
        }

        Ok(())
    }
}
impl WithClaims for FindUnoccupiedRoomsInput {
    fn with_claims(self, _claims: crate::security::Claims) -> Self {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct FindUnoccupiedRoomsOutput {
    pub room_ids: Vec<Uuid>,
}
