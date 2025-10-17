use bevy::prelude::*;
use crate::resources::DatabaseConnection;
use crate::ai::integration::{generate_and_store_map, load_map_into_world};

#[derive(Resource, Default)]
pub struct MapSeed(pub i64);

pub fn init_map_system(mut commands: Commands, db: Res<DatabaseConnection>, seed: Res<MapSeed>) {
    generate_and_store_map(seed.0, &db);
    load_map_into_world(seed.0, &db, commands);
}
