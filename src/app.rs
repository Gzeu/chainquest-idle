use bevy::prelude::*;
use crate::game_plugin::GamePlugin;

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
