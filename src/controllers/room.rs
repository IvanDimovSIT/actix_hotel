use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;

use crate::{
    api::add_room::{AddRoomInput, AddRoomOutput, BedInput},
    app_state::AppState,
    persistence::{bed::BedSize, room::BathroomType, user::Role},
    security::decode_claims,
    services::{add_room::add_room, ErrorReponse},
    validation::Validate,
};

#[derive(OpenApi)]
#[openapi(
    paths(add_room_controller),
    components(schemas(
        ErrorReponse,
        BedInput,
        AddRoomInput,
        AddRoomOutput,
        BathroomType,
        BedSize
    ))
)]
pub struct RoomApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(add_room_controller);
}

#[utoipa::path(
    responses(
        (status = 201, description = "Successfully added room", body = AddRoomOutput),
        (status = 400, description = "Invalid input", body = ErrorReponse),
        (status = 401, description = "Invalid credentials", body = ErrorReponse),
        (status = 403, description = "Invalid credentials", body = ErrorReponse),
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
