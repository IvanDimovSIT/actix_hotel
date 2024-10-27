use actix_web::{body::BoxBody, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::validation::{Validate, Validator};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromoteInput {
    #[schema(example = "user@example.com", required = true)]
    pub email: String,
}
impl Validate for PromoteInput {
    fn validate(&self, validator: &Validator) -> Result<(), HttpResponse<BoxBody>> {
        if let Err(err) = validator.validate_email(&self.email) {
            return Err(err);
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromoteOutput;
