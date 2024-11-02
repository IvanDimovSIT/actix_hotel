use std::collections::HashSet;

use actix_web::{body::BoxBody, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    persistence::{bed::BedSize, room::BathroomType},
    validation::{Validate, Validator},
};

const MIN_BED_COUNT: i16 = 1;
const MAX_BED_COUNT: i16 = 10;

const MIN_FLOOR: i16 = 1;
const MAX_FLOOR: i16 = 100;

const MIN_PRICE: i64 = 1;
const MAX_PRICE: i64 = 100_000_00;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BedInput {
    #[schema(default = "Single", required = true)]
    pub bed_size: BedSize,
    #[schema(example = "2", minimum = 1, maximum = 10, required = true)]
    pub count: i16,
}
impl Validate for BedInput {
    fn validate(&self, _validator: &Validator) -> Result<(), HttpResponse<BoxBody>> {
        if !(MIN_BED_COUNT..=MAX_BED_COUNT).contains(&self.count) {
            return Err(HttpResponse::BadRequest().body(format!(
                "Bed count needs to be between {} and {}",
                MIN_BED_COUNT, MAX_BED_COUNT
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddRoomInput {
    pub beds: Vec<BedInput>,
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
    fn validate_unique(beds: &[BedInput]) -> Result<(), HttpResponse<BoxBody>> {
        let sizes: HashSet<_> = beds.iter().map(|bed| bed.bed_size.clone()).collect();

        if sizes.len() != beds.len() {
            return Err(HttpResponse::BadRequest().body("Bed sizes must not repeat"));
        }

        Ok(())
    }
}
impl Validate for AddRoomInput {
    fn validate(&self, validator: &Validator) -> Result<(), HttpResponse<BoxBody>> {
        Self::validate_unique(&self.beds)?;
        if self.beds.is_empty() {
            return Err(HttpResponse::BadRequest().body("Room needs at least 1 bed"));
        }

        for bed in &self.beds {
            bed.validate(validator)?;
        }

        if !(MIN_FLOOR..=MAX_FLOOR).contains(&self.floor) {
            return Err(HttpResponse::BadRequest().body(format!(
                "Floor need to be between {} and {}",
                MIN_FLOOR, MAX_FLOOR
            )));
        }

        if !(MIN_PRICE..=MAX_PRICE).contains(&self.price) {
            return Err(HttpResponse::BadRequest().body(format!(
                "Price need to be between {} and {}",
                MIN_PRICE, MAX_PRICE
            )));
        }

        validator.validate_room_number(&self.room_number)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddRoomOutput {
    pub room_id: Uuid,
}
