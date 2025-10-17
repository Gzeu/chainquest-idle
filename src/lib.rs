use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::systems_idle::update_idle_progress;
use crate::systems_setup::{setup_camera, setup_ui, setup_map};

/// Render UI elements (simple gizmos)
pub fn render_ui(
    mut gizmos: Gizmos,
    query: Query<&IdleProgress, With<Player>>,
) {
    if let Ok(progress) = query.get_single() {
        let resource_bar_length = (progress.resources / 100.0).min(200.0);
        gizmos.line_2d(
            Vec2::new(-300.0, 300.0),
            Vec2::new(-300.0 + resource_bar_length, 300.0),
            Color::GREEN,
        );
        for i in 0..progress.level {
            gizmos.circle_2d(
                Vec2::new(-280.0 + (i as f32 * 20.0), 250.0),
                8.0,
                Color::YELLOW,
            );
        }
    }
}

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
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameState::default())
            .insert_resource(DatabaseConnection::new())
            .add_systems(Startup, (setup_camera, setup_ui, setup_map))
            .add_systems(Update, (update_idle_progress, handle_input, render_ui));
    }
}

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
