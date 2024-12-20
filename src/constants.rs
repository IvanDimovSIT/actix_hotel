use log::LevelFilter;

pub const REST_HOST: (&str, u16) = ("127.0.0.1", 8080);

pub const APP_DEFAULT_LOGGING_LEVEL: &str = "info";
pub const DB_LOGGING_LEVEL: LevelFilter = LevelFilter::Debug;

pub const BCRYPT_COST: u32 = 10;
pub const BEARER_PREFIX: &str = "Bearer ";
pub const OTP_LENGTH: usize = 8;

pub const ENV_INITIAL_ADMIN_EMAIL: &str = "INITIAL_ADMIN_EMAIL";
pub const ENV_INITIAL_ADMIN_PASSWORD: &str = "INITIAL_ADMIN_PASSWORD";
pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_JWT_SECRET: &str = "JWT_SECRET";
pub const ENV_JWT_VALIDITY_SECS: &str = "JWT_VALIDITY_SECS";
pub const ENV_OTP_VALIDITY_SECS: &str = "OTP_VALIDITY_SECS";
pub const ENV_EMAIL_RELAY: &str = "EMAIL_RELAY";
pub const ENV_EMAIL_USERNAME: &str = "EMAIL_USERNAME";
pub const ENV_EMAIL_PASSWORD: &str = "EMAIL_PASSWORD";

pub const API_NAME: &str = "Hotel API";
pub const API_VERSION: &str = "0.1.0";
pub const API_DESCRIPTION: &str = "Hotel backend system made in the actix web framework.";
