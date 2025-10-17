use bevy::prelude::*;
use crate::config::env::EnvConfig;
use crate::multiplayer::client::{NetConfig};

pub fn apply_env(mut commands: Commands) {
    let cfg = EnvConfig::from_env();
    commands.insert_resource(NetConfig { host: cfg.host, port: cfg.port });
}
