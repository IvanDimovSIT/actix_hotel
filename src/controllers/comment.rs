use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    api::{
        comment::{
            add_comment::{AddCommentInput, AddCommentOutput},
            get_comments::{GetCommentsInput, GetCommentsOutput},
            Comment,
        },
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::Role,
    services::comment::{add_comment::add_comment, get_comments::get_comments},
    util::{process_request, process_request_secured},
};

#[derive(OpenApi)]
#[openapi(
    paths(add_comment_controller),
    components(schemas(
        ErrorResponse,
        AddCommentInput,
        AddCommentOutput,
        Comment,
        GetCommentsInput,
        GetCommentsOutput
    ))
)]
pub struct CommentApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(add_comment_controller);
    cfg.service(get_comments_controller);
}

#[utoipa::path(
    responses(
        (status = 201, description = "Successfully added comment", body = AddCommentOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Invalid credentials", body = ErrorResponse),
        (status = 404, description = "Room not found", body = ErrorResponse),
    ),
    request_body(
        content = AddCommentInput,
        description = "Comment data",
        content_type = "application/json"
    ),
    security(("bearer_auth" = []))
)]
#[post("/comment")]
pub async fn add_comment_controller(
    req: HttpRequest,
    state: Data<AppState>,
    input: Json<AddCommentInput>,
) -> impl Responder {
    process_request_secured(
        req,
        &[Role::User, Role::Admin],
        &state,
        input.into_inner(),
        add_comment,
        StatusCode::CREATED,
    )
    .await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully fetched comments", body = AddCommentOutput),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 404, description = "Room not found", body = ErrorResponse),
    ),
    params(
        ("roomId" = String, Path, description = "Room id", example = "9ddcc342-b0fe-4e1f-a35e-593cb792b55c"),
        ("page" = u64, Query, description = "Page index", example = "1"),
        ("size" = u64, Query, description = "Number of comments to retrieve", example = "10"),
    ),
)]
#[get("/comment/{roomId}")]
pub async fn get_comments_controller(
    state: Data<AppState>,
    input: Query<GetCommentsInput>,
    path: Path<Uuid>,
) -> impl Responder {
    process_request(
        &state,
        GetCommentsInput {
            room_id: Some(path.into_inner()),
            ..input.into_inner()
        },
        get_comments,
        StatusCode::OK,
    )
    .await
}
