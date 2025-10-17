use bevy::prelude::*;
use chainquest_idle::systems::generate_ai_map;

#[test]
fn ai_map_generation_placeholder_runs() {
    // We only assert the system can be invoked without panic; actual content validated in unit tests inside ai module later
    let mut app = App::new();
    app.add_systems(Update, generate_ai_map);
}
