use bevy::prelude::*;
use std::env;

#[derive(Resource, Default, Clone)]
pub struct EnvConfig {
    pub host: String,
    pub port: u16,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        let host = env::var("CQ_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = env::var("CQ_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
        Self { host, port }
    }
}
