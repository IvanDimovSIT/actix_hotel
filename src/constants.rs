pub const REST_HOST: (&str, u16) = ("127.0.0.1", 8080);

pub const APPLICATION_JSON: &str = "application/json";
pub const SALT_LENGTH: usize = 32;
pub const ENV_INITIAL_ADMIN_EMAIL: &str = "INITIAL_ADMIN_EMAIL";
pub const ENV_INITIAL_ADMIN_PASSWORD: &str = "INITIAL_ADMIN_PASSWORD";
pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_JWT_SECRET: &str = "JWT_SECRET";
pub const ENV_JWT_VALIDITY_SECS: &str = "JWT_VALIDITY_SECS";
pub const BEARER_PREFIX: &str = "Bearer ";
