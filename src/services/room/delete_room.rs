use actix_web::http::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel};

use crate::{
    api::{
        error_response::ErrorResponse,
        room::delete_room::{DeleteRoomInput, DeleteRoomOutput},
    },
    app_state::AppState,
    persistence::room,
    util::require_some,
};

pub async fn delete_room_service(
    app_state: &AppState,
    input: DeleteRoomInput,
) -> Result<DeleteRoomOutput, ErrorResponse> {
    let room = find_room(app_state, &input).await?;

    Ok(set_delete_flag_for_room(app_state, room).await?)
}

async fn find_room(
    app_state: &AppState,
    input: &DeleteRoomInput,
) -> Result<room::Model, ErrorResponse> {
    let room_option = room::Entity::find_by_id(input.room_id)
        .one(app_state.db.as_ref())
        .await?;

    let room = require_some(
        room_option,
        || format!("Room with id '{}' not found", input.room_id),
        StatusCode::NOT_FOUND,
    )?;

    if room.is_deleted {
        return Err(ErrorResponse::new(
            format!("Room with id '{}' already deleted", input.room_id),
            StatusCode::NOT_FOUND,
        ));
    }

    Ok(room)
}

async fn set_delete_flag_for_room(
    app_state: &AppState,
    room: room::Model,
) -> Result<DeleteRoomOutput, ErrorResponse> {
    let active_model_room = room::ActiveModel {
        is_deleted: ActiveValue::Set(true),
        ..room.into_active_model()
    };

    active_model_room.update(app_state.db.as_ref()).await?;

    Ok(DeleteRoomOutput)
}
