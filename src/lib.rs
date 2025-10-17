//! ChainQuest Idle - RPG blockchain idle game
//! 
//! Features:
//! - Idle RPG mechanics with Bevy ECS
//! - Multiplayer co-op (ENet)
//! - AI-generated maps and quests (torch-rs) 
//! - MultiversX SFT integration
//! - SQLite local state management

pub mod components;
pub mod systems;
pub mod resources;
pub mod ai;
pub mod blockchain;
pub mod multiplayer;
pub mod utils;

use bevy::prelude::*;
use components::*;
use systems::*;
use resources::*;

/// Main game plugin that sets up all systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resources
            .insert_resource(GameState::default())
            .insert_resource(DatabaseConnection::new())
            
            // Add startup systems
            .add_systems(Startup, (
                setup_camera,
                setup_ui,
                load_saved_progress,
            ))
            
            // Add update systems
            .add_systems(Update, (
                update_idle_progress,
                handle_input,
                render_ui,
                save_progress,
            ));
    }
}

/// Initialize the complete game
pub fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ChainQuest Idle - MVP".into(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}