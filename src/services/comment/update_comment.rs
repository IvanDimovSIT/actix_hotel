use actix_web::http::StatusCode;
use sea_orm::{
    sqlx::types::chrono::Utc, ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel,
};

use crate::{
    api::{
        comment::update_comment::{UpdateCommentInput, UpdateCommentOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::{comment, user::Role},
    util::require_some,
};

async fn find_comment(
    app_state: &AppState,
    input: &UpdateCommentInput,
) -> Result<comment::Model, ErrorResponse> {
    let comment = comment::Entity::find_by_id(input.comment_id)
        .one(app_state.db.as_ref())
        .await?;

    Ok(require_some(
        comment,
        || format!("Comment with id '{}' not found", input.comment_id),
        StatusCode::NOT_FOUND,
    )?)
}

fn check_can_update_comment(
    comment: &comment::Model,
    input: &UpdateCommentInput,
) -> Result<(), ErrorResponse> {
    if matches!(input.role.clone().unwrap(), Role::Admin) {
        return Ok(());
    }

    if comment.user_id == input.user_id.clone().unwrap() {
        Ok(())
    } else {
        let message = "Comment not posted by user".to_owned();
        Err(ErrorResponse::new(message, StatusCode::FORBIDDEN))
    }
}

async fn save_comment(
    app_state: &AppState,
    input: UpdateCommentInput,
    comment: comment::Model,
) -> Result<UpdateCommentOutput, ErrorResponse> {
    let updated_comment = comment::ActiveModel {
        content: ActiveValue::Set(input.contents),
        updated_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
        ..comment.into_active_model()
    };

    updated_comment.save(app_state.db.as_ref()).await?;

    Ok(UpdateCommentOutput)
}

pub async fn update_comment(
    app_state: &AppState,
    input: UpdateCommentInput,
) -> Result<UpdateCommentOutput, ErrorResponse> {
    let comment = find_comment(app_state, &input).await?;
    check_can_update_comment(&comment, &input)?;
    save_comment(app_state, input, comment).await
}
