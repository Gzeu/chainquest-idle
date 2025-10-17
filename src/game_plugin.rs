use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

use crate::components::*;
use crate::resources::*;
use crate::systems_idle::update_idle_progress;
use crate::systems_setup::{setup_camera, setup_ui, setup_map};
use crate::multiplayer::client::{net_setup, net_connect, net_service, net_ping};
use crate::ui::hud::{ui_setup, ui_update};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameState::default())
            .insert_resource(DatabaseConnection::new())
            .add_systems(Startup, (setup_camera, setup_ui, setup_map, net_setup, ui_setup))
            .add_systems(Update, (
                update_idle_progress,
                ui_update,
                net_connect,
                net_service,
                net_ping.run_if(on_timer(Duration::from_millis(1000))),
            ));
    }
}
