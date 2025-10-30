use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub pow_default_prefix: String,
    pub pow_challenge_ttl_seconds: u64,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://haich2.db".to_string());
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()?;
            
        let pow_default_prefix = env::var("POW_DEFAULT_PREFIX")
            .unwrap_or_else(|_| "21e8".to_string());
            
        let pow_challenge_ttl_seconds = env::var("POW_CHALLENGE_TTL_SECONDS")
            .unwrap_or_else(|_| "300".to_string())
            .parse()?;

        Ok(Config {
            database_url,
            port,
            pow_default_prefix,
            pow_challenge_ttl_seconds,
        })
    }
}