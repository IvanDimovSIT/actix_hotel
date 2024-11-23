use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    api::{
        booking::find_unoccupied_rooms::{FindUnoccupiedRoomsInput, FindUnoccupiedRoomsOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::{
        self, bed,
        booking::{self, is_room_occupied_for_period},
        room,
    },
};

async fn find_rooms(app_state: &AppState) -> Result<Vec<room::Model>, ErrorResponse> {
    Ok(room::Entity::find()
        .filter(room::Column::IsDeleted.eq(false))
        .all(app_state.db.as_ref())
        .await?)
}

async fn check_bed_capacity(
    app_state: &AppState,
    room: &room::Model,
    input: &FindUnoccupiedRoomsInput,
) -> Result<bool, ErrorResponse> {
    if input.minimum_capacity.is_none() && input.maximum_capacity.is_none() {
        return Ok(true);
    }

    let beds = room
        .find_related(bed::Entity)
        .all(app_state.db.as_ref())
        .await?;

    let capacity: i16 = beds.iter().map(|bed| bed.total_capacity).sum();

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
    rooms: Vec<room::Model>,
) -> Result<Vec<Uuid>, ErrorResponse> {
    let mut free_room_ids = Vec::with_capacity(100);
    for room in rooms {
        if is_room_occupied_for_period(
            app_state.db.as_ref(),
            room.id,
            input.start_date,
            input.end_date,
        )
        .await?
        {
            continue;
        }
        if !check_bed_capacity(app_state, &room, input).await? {
            continue;
        }

        free_room_ids.push(room.id);
    }

    Ok(free_room_ids)
}

pub async fn find_unoccupied_rooms(
    app_state: &AppState,
    input: FindUnoccupiedRoomsInput,
) -> Result<FindUnoccupiedRoomsOutput, ErrorResponse> {
    let rooms = find_rooms(app_state).await?;
    let room_ids = apply_filters(app_state, &input, rooms).await?;

    let output = FindUnoccupiedRoomsOutput { room_ids };

    Ok(output)
}
