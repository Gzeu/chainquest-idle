//! Game systems for Bevy ECS

use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::components::*;
use crate::resources::*;

/// Setup camera for the game
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Setup initial UI
pub fn setup_ui(mut commands: Commands) {
    // Spawn player entity
    commands.spawn((
        Player,
        IdleProgress::default(),
        Position { x: 0.0, y: 0.0 },
    ));
    
    info!("Game UI initialized");
}

/// Load saved progress from database
pub fn load_saved_progress(
    mut commands: Commands,
    db: Res<DatabaseConnection>,
) {
    if let Ok(progress) = db.load_progress() {
        info!("Loaded saved progress: {} resources", progress.resources);
        // Update existing player or create new one
        commands.spawn((
            Player,
            progress,
            Position { x: 0.0, y: 0.0 },
        ));
    }
}

/// Main idle progress update system
pub fn update_idle_progress(
    mut query: Query<&mut IdleProgress, With<Player>>,
    time: Res<Time>,
) {
    for mut progress in query.iter_mut() {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        let delta_time = current_time - progress.last_update;
        if delta_time > 0.0 {
            // Idle resource generation
            let resource_rate = (progress.level as f32) * 0.5; // Resources per second
            progress.resources += resource_rate * delta_time as f32;
            
            // Experience gain
            progress.experience += 0.1 * delta_time as f32;
            
            // Level up check
            let required_exp = (progress.level * progress.level) as f32 * 10.0;
            if progress.experience >= required_exp {
                progress.level += 1;
                progress.experience = 0.0;
                info!("Level up! New level: {}", progress.level);
            }
            
            progress.last_update = current_time;
        }
    }
}

/// Handle user input
pub fn handle_input(
    mut query: Query<&mut IdleProgress, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut progress in query.iter_mut() {
            progress.resources += 10.0 * (progress.level as f32);
            info!("Manual resource collection! Total: {}", progress.resources);
        }
    }
    
    if keyboard.just_pressed(KeyCode::KeyQ) {
        info!("Quest system activated (placeholder)");
        // TODO: Implement quest generation
    }
}

/// Render UI elements
pub fn render_ui(
    mut gizmos: Gizmos,
    query: Query<&IdleProgress, With<Player>>,
) {
    // Simple visual feedback using gizmos
    if let Ok(progress) = query.get_single() {
        // Draw resource indicator
        let resource_bar_length = (progress.resources / 100.0).min(200.0);
        gizmos.line_2d(
            Vec2::new(-300.0, 300.0),
            Vec2::new(-300.0 + resource_bar_length, 300.0),
            Color::GREEN,
        );
        
        // Draw level indicator
        for i in 0..progress.level {
            gizmos.circle_2d(
                Vec2::new(-280.0 + (i as f32 * 20.0), 250.0),
                8.0,
                Color::YELLOW,
            );
        }
    }
}

/// Save progress to database
pub fn save_progress(
    query: Query<&IdleProgress, With<Player>>,
    db: Res<DatabaseConnection>,
    mut timer: Local<f32>,
    time: Res<Time>,
) {
    *timer += time.delta_seconds();
    
    // Save every 10 seconds
    if *timer >= 10.0 {
        if let Ok(progress) = query.get_single() {
            if let Err(e) = db.save_progress(progress) {
                error!("Failed to save progress: {}", e);
            } else {
                info!("Progress saved: {} resources, level {}", progress.resources, progress.level);
            }
        }
        *timer = 0.0;
    }
}

/// Generate AI map system (placeholder)
pub fn generate_ai_map(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        info!("Generating AI map...");
        
        // Simple 16x16 grid generation
        for x in 0..16 {
            for y in 0..16 {
                let tile_type = match (x + y) % 4 {
                    0 => TileType::Empty,
                    1 => TileType::Resource,
                    2 => TileType::Enemy,
                    3 => TileType::Quest,
                    _ => TileType::Portal,
                };
                
                commands.spawn(MapTile {
                    tile_type,
                    grid_x: x,
                    grid_y: y,
                });
            }
        }
        
        info!("AI map generated: 16x16 grid");
    }
}