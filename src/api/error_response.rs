use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct ErrorReponse {
    error: String,
}
impl ErrorReponse {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}
