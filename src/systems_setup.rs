use bevy::prelude::*;
use crate::ai::startup::{init_map_system, MapSeed};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_ui(mut commands: Commands) {
    use crate::components::{Player, IdleProgress, Position};
    commands.spawn((
        Player,
        IdleProgress::default(),
        Position { x: 0.0, y: 0.0 },
    ));
    info!("Game UI initialized");
}

pub fn setup_map(mut commands: Commands, db: Res<crate::resources::DatabaseConnection>) {
    init_map_system(commands, db, Res::from(MapSeed(1337)));
}
