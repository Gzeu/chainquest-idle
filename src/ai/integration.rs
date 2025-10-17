use bevy::prelude::*;
use crate::resources::{AIState, DatabaseConnection};
use crate::components::{MapTile, TileType};
use crate::ai::mod_stub;

pub fn generate_and_store_map(seed: i64, db: &DatabaseConnection) {
    let grid = mod_stub::generate_map(seed);
    // serialize to simple CSV-like string
    let serialized = grid.iter()
        .map(|row| row.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","))
        .collect::<Vec<_>>()
        .join("\n");
    let _ = db.save_map(seed, &serialized);
}

pub fn load_map_into_world(seed: i64, db: &DatabaseConnection, mut commands: Commands) {
    if let Ok(serialized) = db.load_map(seed) {
        for (y, line) in serialized.lines().enumerate() {
            for (x, cell) in line.split(',').enumerate() {
                let val: i32 = cell.parse().unwrap_or(0);
                let tile_type = match val { 0 => TileType::Empty, 1 => TileType::Resource, 2 => TileType::Enemy, 3 => TileType::Quest, _ => TileType::Empty };
                commands.spawn(MapTile { tile_type, grid_x: x as i32, grid_y: y as i32 });
            }
        }
    }
}
