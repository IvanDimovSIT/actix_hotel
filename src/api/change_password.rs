use actix_web::{body::BoxBody, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::validation::{Validate, Validator};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordInput {
    #[serde(skip)]
    pub user_id: Uuid,
    #[schema(example = "12345678", required = true)]
    pub old_password: String,
    #[schema(example = "12345678", required = true)]
    pub new_password: String,
}
impl Validate for ChangePasswordInput {
    fn validate(&self, validator: &Validator) -> Result<(), HttpResponse<BoxBody>> {
        validator.validate_password(&self.old_password)?;
        validator.validate_password(&self.new_password)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordOutput;
