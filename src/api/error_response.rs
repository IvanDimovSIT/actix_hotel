use std::error::Error;

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::DbErr;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{persistence::handle_db_error, util::serialize_output};

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip)]
    pub status: StatusCode,
}
impl ErrorResponse {
    pub fn new(error: String, status: StatusCode) -> Self {
        Self { error, status }
    }
}
impl From<DbErr> for ErrorResponse {
    fn from(value: DbErr) -> Self {
        handle_db_error(value)
    }
}
impl Into<HttpResponse<BoxBody>> for ErrorResponse {
    fn into(self) -> HttpResponse<BoxBody> {
        let status = self.status;
        serialize_output::<()>(Err(self), status)
    }
}
