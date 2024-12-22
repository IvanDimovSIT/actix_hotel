use std::time::Duration;

use actix_web::rt::time::interval;
use log::{error, info};

use crate::{app_state::AppState, persistence::invalidated_token};

pub struct InvalidatedJwtRemover {
    app_state: AppState,
}
impl InvalidatedJwtRemover {
    async fn remove_old_tokens(app_state: AppState) {
        match invalidated_token::remove_old(
            app_state.db.as_ref(),
            app_state.security_info.jwt_validity,
        )
        .await
        {
            Ok(_) => info!("Removed invalidated tokens"),
            Err(err) => error!("Error removing invalidated tokens: {err}"),
        }
    }

    fn start(app_state: AppState) {
        actix_web::rt::spawn(async move {
            let remover = Self { app_state };
            let mut interval = interval(Duration::from_secs(
                remover.app_state.security_info.jwt_validity,
            ));
            loop {
                interval.tick().await;
                Self::remove_old_tokens(remover.app_state.clone()).await;
            }
        });
    }
}

pub fn start_cronjobs(app_state: AppState) {
    InvalidatedJwtRemover::start(app_state);
    info!("Initialised cronjobs");
}
