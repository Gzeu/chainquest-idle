use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn update_idle_progress(
    mut query: Query<&mut IdleProgress, With<Player>>,
    time: Res<Time>,
) {
    for mut progress in query.iter_mut() {
        let delta = time.delta_seconds_f64();
        if progress.last_update == 0.0 { progress.last_update = time.elapsed_seconds_f64(); }
        let resource_rate = (progress.level as f32) * 0.5;
        progress.resources += resource_rate * delta as f32;
        progress.experience += 0.1 * delta as f32;
        let required_exp = (progress.level * progress.level) as f32 * 10.0;
        if progress.experience >= required_exp {
            progress.level += 1;
            progress.experience = 0.0;
            info!("Level up! New level: {}", progress.level);
        }
        progress.last_update += delta;
    }
}
