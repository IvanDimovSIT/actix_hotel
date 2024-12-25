use actix_web::{
    http::StatusCode,
    post,
    web::{Data, Json, ServiceConfig},
    HttpRequest, Responder,
};
use utoipa::OpenApi;

use crate::{
    api::{
        comment::add_comment::{AddCommentInput, AddCommentOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::Role,
    services::comment::add_comment::add_comment,
    util::process_request_secured,
};

#[derive(OpenApi)]
#[openapi(
    paths(add_comment_controller),
    components(schemas(ErrorResponse, AddCommentInput, AddCommentOutput))
)]
pub struct CommentApiDoc;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(add_comment_controller);
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
