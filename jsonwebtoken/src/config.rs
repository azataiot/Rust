use dotenv::dotenv;
use std::num::ParseIntError;
use std::{env, sync::OnceLock};
use thiserror::Error;

const DEFAULT_JWT_TTL_SECS: i64 = 86_400;

#[derive(Debug)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_ttl_secs: i64,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing env var: {0}")]
    Missing(&'static str),
    #[error("invalid env var {0}: {1}")]
    Invalid(&'static str, #[source] ParseIntError),
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let secret = env::var("JWT_SECRET").map_err(|_| ConfigError::Missing("JWT_SECRET"))?;
        if secret.is_empty() {
            return Err(ConfigError::Missing("JWT_SECRET (empty)"));
        }
        // If env var missing -> default; if present but invalid -> error
        let ttl = match env::var("JWT_TTL_SECS") {
            Ok(s) => s
                .parse()
                .map_err(|e| ConfigError::Invalid("JWT_TTL_SECS", e))?,
            Err(_) => DEFAULT_JWT_TTL_SECS,
        };

        Ok(Self {
            jwt_secret: secret,
            jwt_ttl_secs: ttl,
        })
    }
}

// Global, lazily initialized
static CONFIG: OnceLock<Config> = OnceLock::new();
pub fn get() -> &'static Config {
    CONFIG.get_or_init(|| Config::new().expect("failed to load Config"))
}
