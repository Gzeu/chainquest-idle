use bevy::prelude::*;
use bevy::text::Text2dBounds;
use crate::resources::GameState;
use crate::components::IdleProgress;
use crate::multiplayer::client::NetState;

#[derive(Component)]
pub struct Hud;

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        Hud,
        Text2dBundle {
            text: Text::from_section(
                "ChainQuest HUD",
                TextStyle { font: font.clone(), font_size: 24.0, color: Color::WHITE }
            ),
            text_2d_bounds: Text2dBounds { size: Vec2::new(800.0, 200.0) },
            transform: Transform::from_xyz(-480.0, 340.0, 0.0),
            ..default()
        },
    ));
}

pub fn ui_update(
    mut q: Query<&mut Text, With<Hud>>,
    progress: Query<&IdleProgress>,
    net: Res<NetState>,
    gs: Res<GameState>,
) {
    if let Ok(mut text) = q.get_single_mut() {
        let p = progress.get_single().ok();
        let res = p.map(|v| v.resources).unwrap_or(0.0);
        let lvl = p.map(|v| v.level).unwrap_or(1);
        let conn = if net.connected { "online" } else { "offline" };
        text.sections[0].value = format!(
            "ChainQuest\nResurse: {:.1} | Level: {}\nMultiplayer: {} | Last: {}\nPlayers: {}",
            res, lvl, conn, net.last_msg, gs.total_players
        );
    }
}
