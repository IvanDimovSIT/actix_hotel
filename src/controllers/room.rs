use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;

use crate::{
    api::add_room::{AddRoomInput, AddRoomOutput, BedInput},
    app_state::AppState,
    persistence::user::Role,
    security::decode_claims,
    services::add_room::add_room,
    validation::Validate,
};

#[derive(OpenApi)]
#[openapi(
    paths(add_room_controller),
    components(schemas(BedInput, AddRoomInput, AddRoomOutput))
)]
pub struct RoomApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(add_room_controller);
}

#[utoipa::path(
    responses(
        (status = 201, description = "Successfully added room", body = AddRoomOutput),
        (status = 400, description = "Invalid input", body = String),
        (status = 401, description = "Invalid credentials", body = String),
        (status = 403, description = "Invalid credentials", body = String),
    ),
    request_body(
        content = AddRoomInput,
        description = "Room data",
        content_type = "application/json"
    ),
    security(("bearer_auth" = []))
)]
#[post("/room")]
pub async fn add_room_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Json<AddRoomInput>,
) -> impl Responder {
    if let Err(err) = decode_claims(&req, &state, &[Role::Admin]) {
        return err;
    }

    let add_room_input = input.into_inner();
    if let Err(err) = add_room_input.validate(&state.validator) {
        return err;
    }

    add_room(&state, &add_room_input).await
}
