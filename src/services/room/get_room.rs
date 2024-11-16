use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::{EntityTrait, RelationTrait};

use crate::{api::room::{get_room::{GetRoomInput, GetRoomOutput}, Bed}, app_state::AppState, persistence::{bed, handle_db_error, room::{self, find_room_by_id}}, services::serialize_output, util::require_some};

async fn find_room(app_state: &AppState, input: &GetRoomInput) -> Result<(room::Model, Vec<bed::Model>), HttpResponse<BoxBody>> {
    let room_result = find_room_by_id(app_state.db.as_ref(), input.room_id).await;

    if let Err(err) = room_result {
        return Err(handle_db_error(err));
    }

    let room_option = room_result.unwrap(); 

    require_some(
        room_option, 
        || format!("Room with id '{}' not found", input.room_id),
        StatusCode::NOT_FOUND
    )
}

fn convert_room_to_output(room_beds: (room::Model, Vec<bed::Model>)) -> HttpResponse<BoxBody> {
    let beds = room_beds.1.into_iter()
        .map(|bed| Bed{
            bed_size: bed.bed_size,
            count: bed.count,
        })
        .collect();
    
    let output = GetRoomOutput {
        id: room_beds.0.id,
        price: room_beds.0.price,
        floor: room_beds.0.floor,
        room_number: room_beds.0.room_number,
        bathroom_type: room_beds.0.bathroom_type,
        beds
    };

    serialize_output(&output, StatusCode::OK)
}

pub async fn get_room(app_state: &AppState, input: &GetRoomInput) -> HttpResponse<BoxBody> {
    find_room(app_state, input).await
        .map(convert_room_to_output)
        .unwrap_or_else(|err| err)
}