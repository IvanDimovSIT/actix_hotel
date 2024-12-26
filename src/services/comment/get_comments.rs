use actix_web::http::StatusCode;
use sea_orm::EntityTrait;

use crate::{
    api::{
        comment::{
            get_comments::{GetCommentsInput, GetCommentsOutput},
            Comment,
        },
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::{comment, room},
    util::require_some,
};

async fn check_room_exists_and_not_deleted(
    app_state: &AppState,
    input: &GetCommentsInput,
) -> Result<(), ErrorResponse> {
    let room_option = room::Entity::find_by_id(input.room_id.unwrap())
        .one(app_state.db.as_ref())
        .await?;

    let message = || format!("Room with id '{}' not found", input.room_id.unwrap());
    let room = require_some(room_option, message, StatusCode::NOT_FOUND)?;
    if room.is_deleted {
        return Err(ErrorResponse::new(message(), StatusCode::NOT_FOUND));
    }

    Ok(())
}

fn convert_comments(comments: Vec<comment::Model>) -> Vec<Comment> {
    comments
        .into_iter()
        .map(|comment| Comment {
            id: comment.id,
            room_id: comment.room_id,
            user_id: comment.user_id,
            content: comment.content,
            posted_time: comment.posted_time,
            updated_time: comment.updated_time,
        })
        .collect()
}

async fn fetch_comments(
    app_state: &AppState,
    input: GetCommentsInput,
) -> Result<GetCommentsOutput, ErrorResponse> {
    let (count, comments) = comment::get_paged_comments(
        app_state.db.as_ref(),
        input.room_id.unwrap(),
        input.page,
        input.size,
    )
    .await?;

    Ok(GetCommentsOutput {
        total_size: count,
        comments: convert_comments(comments),
    })
}

pub async fn get_comments(
    app_state: &AppState,
    input: GetCommentsInput,
) -> Result<GetCommentsOutput, ErrorResponse> {
    check_room_exists_and_not_deleted(app_state, &input).await?;
    fetch_comments(app_state, input).await
}
