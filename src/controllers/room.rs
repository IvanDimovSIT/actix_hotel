use actix_web::{
    delete, get,
    http::StatusCode,
    post,
    web::{Data, Json, Path, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    api::{
        error_response::ErrorResponse,
        room::{
            add_room::{AddRoomInput, AddRoomOutput},
            delete_room::{DeleteRoomInput, DeleteRoomOutput},
            get_room::{GetRoomInput, GetRoomOutput},
            Bed,
        },
    },
    app_state::AppState,
    persistence::{bed::BedSize, room::BathroomType, user::Role},
    services::room::{add_room::add_room, delete_room::delete_room, get_room::get_room},
    util::process_request_secured,
};

#[derive(OpenApi)]
#[openapi(
    paths(add_room_controller, get_room_controller, delete_room_controller),
    components(schemas(
        ErrorResponse,
        Bed,
        AddRoomInput,
        AddRoomOutput,
        GetRoomInput,
        GetRoomOutput,
        DeleteRoomInput,
        DeleteRoomOutput,
        BathroomType,
        BedSize
    ))
)]
pub struct RoomApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(add_room_controller);
    cfg.service(get_room_controller);
    cfg.service(delete_room_controller);
}

#[utoipa::path(
    responses(
        (status = 201, description = "Successfully added room", body = AddRoomOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
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
    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        input.into_inner(),
        add_room,
        StatusCode::CREATED,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully fetched room", body = GetRoomOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 404, description = "Room not found", body = ErrorResponse),
    ),
    params(
        ("roomId" = String, Path, description = "Room id")
    ),
    security(("bearer_auth" = []))
)]
#[get("/room/{roomId}")]
pub async fn get_room_controller(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<Uuid>,
) -> impl Responder {
    let input = GetRoomInput {
        room_id: path.into_inner(),
    };

    process_request_secured(
        req,
        &[Role::User, Role::Admin],
        &state,
        input,
        get_room,
        StatusCode::OK,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully deleted room", body = DeleteRoomOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
        (status = 404, description = "Room not found", body = ErrorResponse),
    ),
    params(
        ("roomId" = String, Path, description = "Room id")
    ),
    security(("bearer_auth" = []))
)]
#[delete("/room/{roomId}")]
pub async fn delete_room_controller(
    req: HttpRequest,
    state: Data<AppState>,
    path: Path<Uuid>,
) -> impl Responder {
    let input = DeleteRoomInput {
        room_id: path.into_inner(),
    };

    process_request_secured(
        req,
        &[Role::Admin],
        &state,
        input,
        delete_room,
        StatusCode::OK,
    )
    .await
}
