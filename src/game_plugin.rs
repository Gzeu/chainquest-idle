use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

use crate::components::*;
use crate::resources::*;
use crate::systems_idle::update_idle_progress;
use crate::systems_setup::{setup_camera, setup_ui, setup_map};
use crate::quest_system::{setup_quest_system, generate_quests, process_quest_completion};
use crate::ai::{setup_ai_map_generator, handle_map_generation};
use crate::security::{setup_security_manager, security_cleanup};
use crate::multiplayer::client::{net_setup, net_connect, net_service, net_ping};
use crate::ui::hud::{ui_setup, ui_update};
use crate::config::startup::apply_env;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameState::default())
            .insert_resource(DatabaseConnection::new())
            .add_systems(Startup, (
                apply_env, 
                setup_camera, 
                setup_ui, 
                setup_map, 
                setup_quest_system,
                setup_ai_map_generator,
                setup_security_manager,
                net_setup, 
                ui_setup
            ))
            .add_systems(Update, (
                update_idle_progress,
                generate_quests,
                process_quest_completion,
                handle_map_generation,
                security_cleanup.run_if(on_timer(Duration::from_secs(300))), // Every 5 minutes
                ui_update,
                net_connect,
                net_service,
                net_ping.run_if(on_timer(Duration::from_millis(1000))),
            ));
    }
}