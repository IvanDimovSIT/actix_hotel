use sea_orm::prelude::{Date, DateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::{error_response::ErrorResponse, guest::GuestIdCard},
    persistence::{booking::BookingStatus, user::Role},
    security::WithClaims,
    validation::{Validate, Validator},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct GetBookingInput {
    pub booking_id: Uuid,
    pub user_id: Option<Uuid>,
    pub role: Option<Role>,
}
impl Validate for GetBookingInput {
    fn validate(&self, _validator: &Validator) -> Result<(), ErrorResponse> {
        Ok(())
    }
}
impl WithClaims for GetBookingInput {
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
pub struct GetBookingOutput {
    pub main_guest: BookingGuest,
    pub other_guests: Vec<BookingGuest>,
    pub room_id: Uuid,
    pub admin_id: Uuid,
    pub user_id: Option<Uuid>,
    pub booking_time: DateTime,
    pub payment_time: Option<DateTime>,
    pub start_date: Date,
    pub end_date: Date,
    pub total_price: i64,
    pub status: BookingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookingGuest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Date,
    pub id_card: Option<GuestIdCard>,
    pub phone_number: Option<String>,
}
