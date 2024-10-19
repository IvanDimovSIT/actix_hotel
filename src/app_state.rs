use std::{collections::HashMap, sync::Arc};

use sea_orm::{Database, DatabaseConnection};

use crate::{constants::ENV_DATABASE_URL, persistence::initialise_db};

pub struct EnvironmentVariables {
    env: HashMap<String, String>
}
impl EnvironmentVariables {
    pub fn load() -> Self {
        Self {
            env: dotenv::vars().collect()
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

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub env: Arc<EnvironmentVariables>
}
impl AppState {
    pub async fn load() -> Self {
        let env = EnvironmentVariables::load();

        let state = Self {
            db: Arc::new(load_databse(&env).await),
            env: Arc::new(env)
        };
        println!("App state initialised");

        state
    }
}


async fn load_databse(env: &EnvironmentVariables) -> DatabaseConnection {
    let database_url = env.get(ENV_DATABASE_URL);
    
    let db = Database::connect(database_url).await
        .expect("Failed to connect to database");

    initialise_db(&db, &env).await;
    println!("Database initilised");
    db
}
