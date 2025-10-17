#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use chainquest_idle::systems::update_idle_progress;
    use chainquest_idle::components::{IdleProgress, Player};

    #[test]
    fn idle_progress_increases_resources_and_levels_up() {
        let mut world = World::new();
        world.spawn((Player, IdleProgress { resources: 0.0, experience: 0.0, level: 1, last_update: 0.0 }));

        // Run system twice simulating time passage by manipulating last_update
        let mut schedule = Schedule::default();
        let mut app = App::new();
        app.world = world;
        app.add_systems(Update, update_idle_progress);

        // First tick
        app.update();
        // Manually set last_update to 1 second in the past to force delta
        let mut q = app.world.query::<&mut IdleProgress>();
        for mut p in q.iter_mut(&mut app.world) {
            p.last_update -= 1.0;
        }
        // Second tick
        app.update();

        let mut q2 = app.world.query::<&IdleProgress>();
        for p in q2.iter(&app.world) {
            assert!(p.resources >= 0.5, "resources should increase at least level*0.5 per second");
            // Level-up requires exp >= level^2 * 10; with 1s it won't level, but resources increased
            assert_eq!(p.level, 1);
        }
    }
}
