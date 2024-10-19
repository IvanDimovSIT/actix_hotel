
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection, Schema};
use user::find_user_by_email;
use uuid::Uuid;

use crate::{app_state::EnvironmentVariables, constants::{ENV_INITIAL_ADMIN_EMAIL, ENV_INITIAL_ADMIN_PASSWORD}, security::{generate_salt, hash_with_salt}};


pub mod user;

async fn initialise_admin(db: &DatabaseConnection, env: &EnvironmentVariables) {
    let email = env.get(ENV_INITIAL_ADMIN_EMAIL).to_string();

    let admin_user_present = find_user_by_email(db, &email).await
        .expect("Error intilising admin user")
        .is_some();

    if admin_user_present {
        return;
    }
    
    let raw_password = env.get(ENV_INITIAL_ADMIN_PASSWORD);
    let salt = generate_salt();
    let password = hash_with_salt(&raw_password, &salt);

    let intital_user = user::ActiveModel{
        id: ActiveValue::Set(Uuid::new_v4()),
        email: ActiveValue::Set(email.clone()),
        salt: ActiveValue::Set(salt),
        password: ActiveValue::Set(password),
        role: ActiveValue::Set(user::Role::Admin),
    };

    intital_user.insert(db).await
        .expect("Error inserting initial admin user into database");

    
    println!("Initilised admin user '{email}' with password: '{raw_password}' (change password immediately)");
}

pub async fn initialise_db(db: &DatabaseConnection, env: &EnvironmentVariables) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let statement = builder.build(schema.create_table_from_entity(user::Entity).if_not_exists());

    let result = db.execute(statement).await;
    if result.is_err() {
        println!("Can't create enitiy:{}", result.unwrap_err())
    }

    initialise_admin(db, env).await;
}