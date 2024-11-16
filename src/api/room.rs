use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    persistence::bed::BedSize,
    validation::{Validate, Validator},
};

use super::error_response::ErrorResponse;

pub mod add_room;
pub mod get_room;

const MIN_BED_COUNT: i16 = 1;
const MAX_BED_COUNT: i16 = 10;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct Bed {
    #[schema(default = "Single", required = true)]
    pub bed_size: BedSize,
    #[schema(example = "2", minimum = 1, maximum = 10, required = true)]
    pub count: i16,
}
impl Validate for Bed {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        if !(MIN_BED_COUNT..=MAX_BED_COUNT).contains(&self.count) {
            return Err(ErrorResponse::new(
                format!(
                    "Bed count needs to be between {} and {}",
                    MIN_BED_COUNT, MAX_BED_COUNT
                ),
                StatusCode::BAD_REQUEST,
            ));
        }

        Ok(())
    }
}
