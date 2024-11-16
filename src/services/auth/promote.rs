use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};

use crate::{
    api::{
        auth::promote::{PromoteInput, PromoteOutput},
        error_response::ErrorResponse,
    },
    app_state::AppState,
    persistence::user::Role,
    util::find_user,
};

pub async fn promote(
    app_state: &AppState,
    input: &PromoteInput,
) -> Result<PromoteOutput, ErrorResponse> {
    let found_user = find_user(app_state, &input.email).await?;
    let mut active_user = found_user.into_active_model();
    active_user.role = ActiveValue::Set(Role::Admin);

    active_user.save(app_state.db.as_ref()).await?;

    Ok(PromoteOutput)
}
