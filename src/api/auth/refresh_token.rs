use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::security::Claims;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct RefreshTokenInput {
    pub claims: Claims,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct RefreshTokenOutput {
    pub token: String,
}
