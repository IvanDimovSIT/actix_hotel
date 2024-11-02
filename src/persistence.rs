use actix_web::{body::BoxBody, HttpResponse};
use env_logger::Logger;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr,
    EntityTrait, Schema,
};
use user::find_user_by_email;
use uuid::Uuid;

use crate::{
    app_state::EnvironmentVariables,
    constants::{ENV_INITIAL_ADMIN_EMAIL, ENV_INITIAL_ADMIN_PASSWORD},
    security::{generate_salt, hash_with_salt},
};

pub mod bed;
pub mod room;
pub mod user;

pub fn handle_db_error(error: DbErr) -> HttpResponse<BoxBody> {
    match error {
        DbErr::ConnectionAcquire(conn_acquire_err) => HttpResponse::InternalServerError()
            .body(format!("DB error: Connection error:{}", conn_acquire_err)),
        DbErr::TryIntoErr { from, into, source } => {
            HttpResponse::InternalServerError().body(format!(
                "DB error: Conversion error from '{}' to '{}':{}",
                from, into, source
            ))
        }
        DbErr::Conn(runtime_err) => HttpResponse::InternalServerError()
            .body(format!("DB error: Connection error:{}", runtime_err)),
        DbErr::Exec(runtime_err) => HttpResponse::InternalServerError().body(format!(
            "DB error: An operation did not execute successfully:{}",
            runtime_err
        )),
        DbErr::Query(runtime_err) => HttpResponse::InternalServerError().body(format!(
            "DB error: An error occurred while performing a query:{}",
            runtime_err
        )),
        DbErr::ConvertFromU64(e) => HttpResponse::InternalServerError().body(format!(
            "DB error: Type error: the specified type cannot be converted from u64:{}",
            e
        )),
        DbErr::UnpackInsertId => HttpResponse::InternalServerError().body(
            "DB error: After an insert statement it was impossible to retrieve the last_insert_id",
        ),
        DbErr::UpdateGetPrimaryKey => {
            HttpResponse::InternalServerError().body("DB error: Update Get Primary Key")
        }
        DbErr::RecordNotFound(e) => HttpResponse::InternalServerError().body(format!(
            "DB error: The record was not found in the database:{}",
            e
        )),
        DbErr::Type(e) => HttpResponse::InternalServerError().body(format!(
            "DB error: Error occurred while parsing value as target type:{}",
            e
        )),
        DbErr::Json(e) => HttpResponse::InternalServerError().body(format!(
            "DB error: Error occurred while parsing json value as target type:{}",
            e
        )),
        DbErr::Migration(e) => {
            HttpResponse::InternalServerError().body(format!("DB error: A migration error:{}", e))
        }
        DbErr::RecordNotInserted => {
            HttpResponse::InternalServerError().body("DB error: Record Not Inserted")
        }
        DbErr::RecordNotUpdated => {
            HttpResponse::InternalServerError().body("DB error: Record Not Updated")
        }
        _ => HttpResponse::InternalServerError().body("DB error"),
    }
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
    let salt = generate_salt();
    let password = hash_with_salt(&raw_password, &salt);

    let intital_user = user::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        email: ActiveValue::Set(email.clone()),
        salt: ActiveValue::Set(salt),
        password: ActiveValue::Set(password),
        role: ActiveValue::Set(user::Role::Admin),
    };

    intital_user
        .insert(db)
        .await
        .expect("Error inserting initial admin user into database");

    println!("Initilised admin user with email: '{email}' and password: '{raw_password}' (change password immediately)");
}

async fn intitialise_table<E>(db: &DatabaseConnection, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let statement = builder.build(schema.create_table_from_entity(entity).if_not_exists());
    let result = db.execute(statement).await;
    if let Err(err) = result {
        panic!("Can't create enitiy:{}", err);
    }
}

pub async fn initialise_db(db: &DatabaseConnection, env: &EnvironmentVariables) {
    intitialise_table(db, user::Entity).await;
    intitialise_table(db, room::Entity).await;
    intitialise_table(db, bed::Entity).await;

    initialise_admin(db, env).await;
}
