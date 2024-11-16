use std::collections::HashSet;

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    persistence::{bed::BedSize, room::BathroomType},
    validation::{Validate, Validator},
};

use super::Bed;

const MIN_FLOOR: i16 = 1;
const MAX_FLOOR: i16 = 100;

const MIN_PRICE: i64 = 1;
const MAX_PRICE: i64 = 100_000_00;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct AddRoomInput {
    pub beds: Vec<Bed>,
    #[schema(example = "5000", minimum = 1, maximum = 100_000_00, required = true)]
    pub price: i64,
    #[schema(example = "3", minimum = 1, maximum = 100, required = true)]
    pub floor: i16,
    #[schema(example = "108A", required = true)]
    pub room_number: String,
    #[schema(default = "Private", required = true)]
    pub bathroom_type: BathroomType,
}
impl AddRoomInput {
    fn validate_unique(beds: &[Bed]) -> Result<(), ErrorResponse> {
        let sizes: HashSet<_> = beds.iter().map(|bed| bed.bed_size.clone()).collect();

        if sizes.len() != beds.len() {
            return Err(ErrorResponse::new(
                "Bed sizes must not repeat".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        Ok(())
    }
}
impl Validate for AddRoomInput {
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse> {
        Self::validate_unique(&self.beds)?;
        if self.beds.is_empty() {
            return Err(ErrorResponse::new(
                "Room needs at least 1 bed".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        for bed in &self.beds {
            bed.validate(validator)?;
        }

        if !(MIN_FLOOR..=MAX_FLOOR).contains(&self.floor) {
            return Err(ErrorResponse::new(
                format!("Floor need to be between {} and {}", MIN_FLOOR, MAX_FLOOR),
                StatusCode::BAD_REQUEST,
            ));
        }

        if !(MIN_PRICE..=MAX_PRICE).contains(&self.price) {
            return Err(ErrorResponse::new(
                format!("Price need to be between {} and {}", MIN_PRICE, MAX_PRICE),
                StatusCode::BAD_REQUEST,
            ));
        }

        validator.validate_room_number(&self.room_number)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct AddRoomOutput {
    pub room_id: Uuid,
}
