use actix_web::http::StatusCode;

use crate::{
    api::{
        error_response::ErrorResponse,
        room::{
            get_room::{GetRoomInput, GetRoomOutput},
            Bed,
        },
    },
    app_state::AppState,
    persistence::{
        bed,
        room::{self, find_room_by_id},
    },
    util::require_some,
};

pub async fn get_room_service(
    app_state: &AppState,
    input: GetRoomInput,
) -> Result<GetRoomOutput, ErrorResponse> {
    find_room(app_state, &input)
        .await
        .map(convert_room_to_output)
}

async fn find_room(
    app_state: &AppState,
    input: &GetRoomInput,
) -> Result<(room::Model, Vec<bed::Model>), ErrorResponse> {
    let room_option = find_room_by_id(app_state.db.as_ref(), input.room_id).await?;

    let room_beds = require_some(
        room_option,
        || format!("Room with id '{}' not found", input.room_id),
        StatusCode::NOT_FOUND,
    );

    match &room_beds {
        Ok((room, _beds)) => {
            if room.is_deleted {
                return Err(ErrorResponse::new(
                    format!("Room with id '{}' not found", input.room_id),
                    StatusCode::NOT_FOUND,
                ));
            }
        }
        _ => {}
    }

    room_beds
}

fn convert_room_to_output(room_beds: (room::Model, Vec<bed::Model>)) -> GetRoomOutput {
    let beds = room_beds
        .1
        .into_iter()
        .map(|bed| Bed {
            bed_size: bed.bed_size,
            count: bed.count,
        })
        .collect();

    GetRoomOutput {
        id: room_beds.0.id,
        price: room_beds.0.price,
        floor: room_beds.0.floor,
        room_number: room_beds.0.room_number,
        bathroom_type: room_beds.0.bathroom_type,
        beds,
    }
}
