use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    validation::{Validate, Validator},
};

use super::Comment;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GetCommentsInput {
    #[serde(skip)]
    pub room_id: Option<Uuid>,
    pub page: u64,
    pub size: u64,
}
impl Validate for GetCommentsInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        Validator::validate_option(&self.room_id, "room_id")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GetCommentsOutput {
    pub total_size: u64,
    pub comments: Vec<Comment>,
}
