use actix_web::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};
use uuid::Uuid;

use crate::{
    api::{
        error_response::ErrorResponse,
        room::{
            add_room::{AddRoomInput, AddRoomOutput},
            Bed,
        },
    },
    app_state::AppState,
    persistence::{
        bed, handle_db_error,
        room::{self},
    },
};

async fn check_room_number_not_used(
    db: &DatabaseConnection,
    input: &AddRoomInput,
) -> Result<(), ErrorResponse> {
    let result = room::find_by_room_number(db, &input.room_number).await;
    if let Err(err) = result {
        return Err(handle_db_error(err));
    }

    if result.unwrap().is_some() {
        return Err(ErrorResponse::new(
            format!("Room number '{}' is already in use", input.room_number),
            StatusCode::BAD_REQUEST,
        ));
    }

    Ok(())
}

async fn insert_room(
    transaction: &DatabaseTransaction,
    input: &AddRoomInput,
) -> Result<Uuid, ErrorResponse> {
    let id = Uuid::new_v4();
    let room_to_save = room::ActiveModel {
        id: ActiveValue::Set(id),
        price: ActiveValue::Set(input.price),
        floor: ActiveValue::Set(input.floor),
        room_number: ActiveValue::Set(input.room_number.clone()),
        bathroom_type: ActiveValue::Set(input.bathroom_type.clone()),
        is_deleted: ActiveValue::Set(false),
    };
    if let Err(err) = room_to_save.insert(transaction).await {
        return Err(handle_db_error(err));
    }

    Ok(id)
}

async fn insert_bed(
    transaction: &DatabaseTransaction,
    input: &Bed,
    room_id: &Uuid,
) -> Result<(), ErrorResponse> {
    let total_capacity = input.bed_size.get_size() * input.count;
    let bed_to_save = bed::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        room_id: ActiveValue::Set(*room_id),
        bed_size: ActiveValue::Set(input.bed_size.clone()),
        count: ActiveValue::Set(input.count),
        total_capacity: ActiveValue::Set(total_capacity),
    };
    bed_to_save.insert(transaction).await?;

    Ok(())
}

pub async fn add_room(
    app_state: &AppState,
    input: &AddRoomInput,
) -> Result<AddRoomOutput, ErrorResponse> {
    check_room_number_not_used(&app_state.db, input).await?;

    let transaction = app_state.db.begin().await?;

    let room_id = insert_room(&transaction, input).await?;

    for bed in &input.beds {
        insert_bed(&transaction, bed, &room_id).await?;
    }

    transaction.commit().await?;

    Ok(AddRoomOutput { room_id })
}
