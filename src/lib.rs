pub mod components;
pub mod systems_idle;
pub mod systems_setup;
pub mod quest_system;
pub mod resources;
pub mod ai { pub mod mod_stub; pub mod integration; pub mod startup; }
pub mod multiplayer { pub mod client; }
pub mod ui { pub mod hud; }
pub mod game_plugin;
pub mod app;
pub mod utils;

pub use app::run_game;