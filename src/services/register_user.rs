use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use uuid::Uuid;

use crate::{api::register_user::{RegisterUserInput, RegisterUserOutput}, persistence::{handle_db_error, user::{self, find_user_by_email}}, security::{generate_salt, hash_with_salt}};

use super::serialize_output;



pub async fn register_user(db: &DatabaseConnection, input: &RegisterUserInput) -> HttpResponse<BoxBody> {
    let result = find_user_by_email(db, &input.email).await;
    if let Err(err) = result {
        return HttpResponse::from_error(err);
    }

    if result.unwrap().is_some() {
        return HttpResponse::BadRequest().body(format!("Email {} already taken", &input.email));
    }

    let salt = generate_salt();
    let password = hash_with_salt(&input.password, &salt);

    let user_to_save = user::ActiveModel{
        id: ActiveValue::Set(Uuid::new_v4()),
        email: ActiveValue::Set(input.email.clone()),
        salt: ActiveValue::Set(salt),
        password: ActiveValue::Set(password),
        role: ActiveValue::Set(user::Role::User),
    };

    let result = user_to_save.insert(db).await;
    if let Err(err) = result {
        return handle_db_error(err);
    }

    let output = RegisterUserOutput{
        user_id: result.unwrap().id
    };

    serialize_output(&output, StatusCode::CREATED)
}