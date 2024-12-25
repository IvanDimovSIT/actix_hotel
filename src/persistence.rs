use actix_web::http::StatusCode;
use log::{error, info};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, Schema,
};
use user::find_user_by_email;
use uuid::Uuid;

use crate::{
    api::error_response::ErrorResponse,
    app_state::EnvironmentVariables,
    constants::{ENV_INITIAL_ADMIN_EMAIL, ENV_INITIAL_ADMIN_PASSWORD},
    security::hash_password,
};

pub mod bed;
pub mod booking;
pub mod booking_guest;
pub mod comment;
pub mod guest;
pub mod invalidated_token;
pub mod one_time_password;
pub mod room;
pub mod user;

fn db_error_to_string(error: DbErr) -> String {
    match error {
        DbErr::ConnectionAcquire(conn_acquire_err) => {
            format!("DB error: Connection error:{}", conn_acquire_err)
        }
        DbErr::TryIntoErr { from, into, source } => {
            format!(
                "DB error: Conversion error from '{}' to '{}':{}",
                from, into, source
            )
        }
        DbErr::Conn(runtime_err) => format!("DB error: Connection error:{}", runtime_err),
        DbErr::Exec(runtime_err) => format!(
            "DB error: An operation did not execute successfully:{}",
            runtime_err
        ),
        DbErr::Query(runtime_err) => format!(
            "DB error: An error occurred while performing a query:{}",
            runtime_err
        ),
        DbErr::ConvertFromU64(e) => format!(
            "DB error: Type error: the specified type cannot be converted from u64:{}",
            e
        ),
        DbErr::UnpackInsertId => {
            "DB error: After an insert statement it was impossible to retrieve the last_insert_id"
                .to_string()
        }
        DbErr::UpdateGetPrimaryKey => "DB error: Update Get Primary Key".to_string(),
        DbErr::RecordNotFound(e) => {
            format!("DB error: The record was not found in the database:{}", e)
        }
        DbErr::Type(e) => format!(
            "DB error: Error occurred while parsing value as target type:{}",
            e
        ),
        DbErr::Json(e) => format!(
            "DB error: Error occurred while parsing json value as target type:{}",
            e
        ),
        DbErr::Migration(e) => {
            format!("DB error: A migration error:{}", e)
        }
        DbErr::RecordNotInserted => "DB error: Record Not Inserted".to_string(),
        DbErr::RecordNotUpdated => "DB error: Record Not Updated".to_string(),
        _ => "DB error".to_string(),
    }
}

pub fn handle_db_error(error: DbErr) -> ErrorResponse {
    error!("Database error: {error}");
    ErrorResponse::new(db_error_to_string(error), StatusCode::INTERNAL_SERVER_ERROR)
}

async fn initialise_admin(db: &DatabaseConnection, env: &EnvironmentVariables) {
    let email = env.get(ENV_INITIAL_ADMIN_EMAIL).to_string();

    let admin_user_present = find_user_by_email(db, &email)
        .await
        .expect("Error intilising admin user")
        .is_some();

    if admin_user_present {
        return;
    }

    let raw_password = env.get(ENV_INITIAL_ADMIN_PASSWORD);
    let password = hash_password(&raw_password);

    let intital_user = user::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        email: ActiveValue::Set(email.clone()),
        password: ActiveValue::Set(password),
        role: ActiveValue::Set(user::Role::Admin),
    };

    intital_user
        .insert(db)
        .await
        .expect("Error inserting initial admin user into database");

    info!("Initialised admin user with email: '{email}' and password: '{raw_password}' (change password immediately)");
}

async fn create_table<E>(db: &DatabaseConnection, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let statement = builder.build(schema.create_table_from_entity(entity).if_not_exists());
    let result = db.execute(statement).await;
    if let Err(err) = result {
        error!("Can't create enitiy:{}", err);
        panic!("Can't create enitiy:{}", err);
    }
}

pub async fn initialise_db(db: &DatabaseConnection, env: &EnvironmentVariables) {
    create_table(db, user::Entity).await;
    create_table(db, room::Entity).await;
    create_table(db, bed::Entity).await;
    create_table(db, one_time_password::Entity).await;
    create_table(db, guest::Entity).await;
    create_table(db, booking::Entity).await;
    create_table(db, booking_guest::Entity).await;
    create_table(db, invalidated_token::Entity).await;
    create_table(db, comment::Entity).await;

    initialise_admin(db, env).await;
}
