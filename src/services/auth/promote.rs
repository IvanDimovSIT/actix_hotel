use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};

use crate::{
    api::auth::promote::{PromoteInput, PromoteOutput},
    app_state::AppState,
    persistence::{handle_db_error, user::Role},
    services::serialize_output,
    util::find_user,
};

pub async fn promote(app_state: &AppState, input: &PromoteInput) -> HttpResponse<BoxBody> {
    let found_user = find_user(app_state, &input.email).await;
    if let Err(err) = found_user {
        return err;
    }
    let mut user = found_user.unwrap().into_active_model();
    user.role = ActiveValue::Set(Role::Admin);

    let result = user.save(app_state.db.as_ref()).await;
    if let Err(err) = result {
        return handle_db_error(err);
    }

    serialize_output(&PromoteOutput, StatusCode::OK)
}
