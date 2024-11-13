use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};
use uuid::Uuid;

use crate::{
    api::room::{
        add_room::{AddRoomInput, AddRoomOutput},
        BedInput,
    },
    app_state::AppState,
    persistence::{
        bed, handle_db_error,
        room::{self},
    },
    services::{error_response, serialize_output},
};

async fn check_room_number_not_used(
    db: &DatabaseConnection,
    input: &AddRoomInput,
) -> Result<(), HttpResponse<BoxBody>> {
    let result = room::find_by_room_number(db, &input.room_number).await;
    if let Err(err) = result {
        return Err(handle_db_error(err));
    }

    if result.unwrap().is_some() {
        return Err(error_response(
            format!("Room number '{}' is already in use", input.room_number),
            StatusCode::BAD_REQUEST,
        ));
    }

    Ok(())
}

async fn insert_room(
    transaction: &DatabaseTransaction,
    input: &AddRoomInput,
) -> Result<Uuid, HttpResponse<BoxBody>> {
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
    input: &BedInput,
    room_id: &Uuid,
) -> Result<(), HttpResponse<BoxBody>> {
    let total_capacity = input.bed_size.get_size() * input.count;
    let bed_to_save = bed::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        room_id: ActiveValue::Set(*room_id),
        bed_size: ActiveValue::Set(input.bed_size.clone()),
        count: ActiveValue::Set(input.count),
        total_capacity: ActiveValue::Set(total_capacity),
    };
    if let Err(err) = bed_to_save.insert(transaction).await {
        return Err(handle_db_error(err));
    }

    Ok(())
}

pub async fn add_room(app_state: &AppState, input: &AddRoomInput) -> HttpResponse<BoxBody> {
    if let Err(err) = check_room_number_not_used(&app_state.db, input).await {
        return err;
    }

    let transaction_result = app_state.db.begin().await;
    if let Err(err) = transaction_result {
        return handle_db_error(err);
    }
    let transaction = transaction_result.unwrap();

    let insert_room_result = insert_room(&transaction, input).await;
    if let Err(err) = insert_room_result {
        return err;
    }

    let room_id = insert_room_result.unwrap();
    for bed in &input.beds {
        if let Err(err) = insert_bed(&transaction, bed, &room_id).await {
            return err;
        }
    }

    if let Err(err) = transaction.commit().await {
        return handle_db_error(err);
    }

    let output = AddRoomOutput { room_id };

    serialize_output(&output, StatusCode::CREATED)
}
