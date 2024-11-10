use std::{collections::HashMap, sync::Arc};

use log::info;
use sea_orm::{Database, DatabaseConnection};

use crate::{
    constants::{ENV_DATABASE_URL, ENV_JWT_SECRET, ENV_JWT_VALIDITY_SECS, ENV_OTP_VALIDITY_SECS},
    persistence::initialise_db,
    services::email_service::MailService,
    validation::Validator,
};

pub struct EnvironmentVariables {
    env: HashMap<String, String>,
}
impl EnvironmentVariables {
    pub fn load() -> Self {
        Self {
            env: dotenv::vars().collect(),
        }
    }

    pub fn get(&self, key: &str) -> &str {
        let val = self.env.get(key);
        if val.is_none() {
            panic!("Environment variable '{}' not found", key);
        }

        val.unwrap()
    }
}

pub struct SecurityInfo {
    pub jwt_secret: String,
    pub jwt_validity: u64,
    pub otp_validity: u64,
}
impl SecurityInfo {
    fn new(env: &EnvironmentVariables) -> Self {
        let jwt_secret = env.get(ENV_JWT_SECRET).to_string();
        let jwt_validity = env.get(ENV_JWT_VALIDITY_SECS).parse().expect(&format!(
            "Invalid number format for {ENV_JWT_VALIDITY_SECS}"
        ));

        let otp_validity = env.get(ENV_OTP_VALIDITY_SECS).parse().expect(&format!(
            "Invalid number format for {ENV_OTP_VALIDITY_SECS}"
        ));

        Self {
            jwt_secret,
            jwt_validity,
            otp_validity,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub validator: Arc<Validator>,
    pub security_info: Arc<SecurityInfo>,
    pub mail_service: Arc<MailService>,
}
impl AppState {
    pub async fn load() -> Self {
        let env = EnvironmentVariables::load();
        let security_info = SecurityInfo::new(&env);
        let mail_service = MailService::new(&env);

        let state = Self {
            db: Arc::new(load_databse(&env).await),
            validator: Arc::new(Validator::new()),
            security_info: Arc::new(security_info),
            mail_service: Arc::new(mail_service),
        };

        state
    }
}

async fn load_databse(env: &EnvironmentVariables) -> DatabaseConnection {
    let database_url = env.get(ENV_DATABASE_URL);

    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    initialise_db(&db, &env).await;
    info!("Database initilised");
    db
}
