use uuid::Uuid;

use crate::{
    api::{
        booking::find_unoccupied_rooms::{FindUnoccupiedRoomsInput, FindUnoccupiedRoomsOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::{bed, booking::is_room_occupied_for_period, room},
};

pub async fn find_unoccupied_rooms_service(
    app_state: &AppState,
    input: FindUnoccupiedRoomsInput,
) -> Result<FindUnoccupiedRoomsOutput, ErrorResponse> {
    let room_ids = room::find_all_room_ids_not_deleted(app_state.db.as_ref()).await?;
    let free_room_ids = apply_filters(app_state, &input, &room_ids).await?;

    let output = FindUnoccupiedRoomsOutput {
        room_ids: free_room_ids,
    };

    Ok(output)
}

async fn check_bed_capacity(
    app_state: &AppState,
    room_id: Uuid,
    input: &FindUnoccupiedRoomsInput,
) -> Result<bool, ErrorResponse> {
    if input.minimum_capacity.is_none() && input.maximum_capacity.is_none() {
        return Ok(true);
    }

    let capacity = bed::find_total_bed_capacity_for_room(app_state.db.as_ref(), room_id).await?;

    if let Some(max) = input.maximum_capacity {
        if capacity > max {
            return Ok(false);
        }
    }
    if let Some(min) = input.minimum_capacity {
        if capacity < min {
            return Ok(false);
        }
    }

    Ok(true)
}

async fn apply_filters(
    app_state: &AppState,
    input: &FindUnoccupiedRoomsInput,
    room_ids: &[Uuid],
) -> Result<Vec<Uuid>, ErrorResponse> {
    let mut free_room_ids = Vec::with_capacity(100);
    for id in room_ids {
        if !check_bed_capacity(app_state, *id, input).await? {
            continue;
        }
        if is_room_occupied_for_period(app_state.db.as_ref(), *id, input.start_date, input.end_date)
            .await?
        {
            continue;
        }

        free_room_ids.push(*id);
    }

    Ok(free_room_ids)
}
