use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    persistence::user::Role,
    security::WithClaims,
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct UpdateCommentInput {
    #[serde(skip)]
    pub user_id: Option<Uuid>,
    #[serde(skip)]
    pub role: Option<Role>,
    #[serde(skip)]
    pub comment_id: Uuid,
    pub contents: String,
}
impl Validate for UpdateCommentInput {
    fn validate(&self, validator: &Validator) -> Result<(), ErrorResponse> {
        Validator::validate_option(&self.user_id, "user_id")?;
        Validator::validate_option(&self.role, "role")?;
        validator.validate_comment_contents(&self.contents)?;

        Ok(())
    }
}
impl WithClaims for UpdateCommentInput {
    fn with_claims(self, claims: crate::security::Claims) -> Self {
        Self {
            user_id: Some(claims.user_id),
            role: Some(claims.role),
            ..self
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct UpdateCommentOutput;
