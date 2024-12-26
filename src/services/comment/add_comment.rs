use actix_web::http::StatusCode;
use sea_orm::{sqlx::types::chrono::Utc, ActiveModelTrait, EntityTrait, IntoActiveModel};
use uuid::Uuid;

use crate::{
    api::{
        comment::add_comment::{AddCommentInput, AddCommentOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::{booking, comment, room},
    util::require_some,
};

async fn check_room_exists(
    app_state: &AppState,
    input: &AddCommentInput,
) -> Result<(), ErrorResponse> {
    let room_option = room::Entity::find_by_id(input.room_id)
        .one(app_state.db.as_ref())
        .await?;

    let message = || format!("Room with id '{}' not found", input.room_id);
    let room = require_some(room_option, message, StatusCode::NOT_FOUND)?;

    if room.is_deleted {
        return Err(ErrorResponse::new(message(), StatusCode::NOT_FOUND));
    }

    Ok(())
}

async fn check_user_can_post(
    app_state: &AppState,
    input: &AddCommentInput,
) -> Result<(), ErrorResponse> {
    match input.role.clone().unwrap() {
        crate::persistence::user::Role::User => {
            let has_booking = booking::user_has_booking_for_room(
                app_state.db.as_ref(),
                input.user_id.unwrap(),
                input.room_id,
            )
            .await?;

            if has_booking {
                Ok(())
            } else {
                Err(ErrorResponse::new(
                    "User doesn't have booking for room".to_owned(),
                    StatusCode::UNAUTHORIZED,
                ))
            }
        }
        crate::persistence::user::Role::Admin => Ok(()),
    }
}

async fn insert_comment(
    app_state: &AppState,
    input: AddCommentInput,
) -> Result<AddCommentOutput, ErrorResponse> {
    let id = Uuid::new_v4();
    comment::Model {
        id,
        room_id: input.room_id,
        user_id: input.user_id.unwrap(),
        content: input.contents,
        posted_time: Utc::now().naive_utc(),
        updated_time: None,
    }
    .into_active_model()
    .insert(app_state.db.as_ref())
    .await?;

    Ok(AddCommentOutput { comment_id: id })
}

pub async fn add_comment(
    app_state: &AppState,
    input: AddCommentInput,
) -> Result<AddCommentOutput, ErrorResponse> {
    check_room_exists(app_state, &input).await?;
    check_user_can_post(app_state, &input).await?;
    insert_comment(app_state, input).await
}
