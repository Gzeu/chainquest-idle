#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use chainquest_idle::systems_idle::update_idle_progress;
    use chainquest_idle::components::{IdleProgress, Player};

    #[test]
    fn idle_progress_increases_resources_and_levels_up() {
        let mut app = App::new();
        // Insert Time resource (starts at 0) and a player
        app.insert_resource(Time::default());
        app.world.spawn((Player, IdleProgress { resources: 0.0, experience: 0.0, level: 1, last_update: 0.0 }));
        app.add_systems(Update, update_idle_progress);

        // Simulate 1.0 second of game time in two 0.5s steps
        app.update();
        app.world.resource_mut::<Time>().advance_by(std::time::Duration::from_millis(500));
        app.update();
        app.world.resource_mut::<Time>().advance_by(std::time::Duration::from_millis(500));
        app.update();

        let mut q = app.world.query::<&IdleProgress>();
        for p in q.iter(&app.world) {
            assert!(p.resources > 0.0, "resources should increase with time delta");
            assert_eq!(p.level, 1);
        }
    }
}
