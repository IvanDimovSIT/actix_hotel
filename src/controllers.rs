use utoipa::openapi::{
    security::{HttpBuilder, SecurityScheme},
    Components, Info, Object, OpenApi, Schema,
};

use crate::constants::{API_DESCRIPTION, API_NAME, API_VERSION};

pub mod auth;
pub mod booking;
pub mod comment;
pub mod guest;
pub mod room;

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    auth::register_controller,
    auth::login_controller,
    auth::promote_controller,
    auth::refresh_token_controller,
    auth::change_password_controller,
    auth::reset_password_controller,
    auth::send_otp_controller,
    auth::logout_controller,
    room::add_room_controller,
    room::get_room_controller,
    room::delete_room_controller,
    guest::add_guest_controller,
    guest::find_guest_controller,
    guest::get_guest_controller,
    guest::update_guest_controller,
    booking::find_unoccupied_rooms_controller,
    booking::book_room_controller,
    booking::pay_booking_controller,
    booking::get_booking_controller,
    booking::get_own_bookings_controller,
    booking::cancel_booking_controller,
    comment::add_comment_controller,
    comment::get_comments_controller,
    comment::update_comment_controller
))]
pub struct ApiDoc;

impl ApiDoc {
    pub fn new() -> OpenApi {
        let mut api = <ApiDoc as utoipa::OpenApi>::openapi();
        let mut componenets = Components::new();
        componenets.schemas.insert(
            "Uuid".to_string(),
            utoipa::openapi::RefOr::T(Schema::Object(Object::new())),
        );
        componenets.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        api.components = Some(componenets);

        api.merge(<auth::AuthApiDoc as utoipa::OpenApi>::openapi());
        api.merge(<room::RoomApiDoc as utoipa::OpenApi>::openapi());
        api.merge(<guest::GuestApiDoc as utoipa::OpenApi>::openapi());
        api.merge(<booking::BookingApiDoc as utoipa::OpenApi>::openapi());
        api.merge(<comment::CommentApiDoc as utoipa::OpenApi>::openapi());
        api.info = Info::new(API_NAME, API_VERSION);
        api.info.description = Some(API_DESCRIPTION.to_string());

        api
    }
}
