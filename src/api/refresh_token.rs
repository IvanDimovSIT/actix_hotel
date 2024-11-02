use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::security::Claims;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenInput {
    pub claims: Claims,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenOutput {
    pub token: String,
}
