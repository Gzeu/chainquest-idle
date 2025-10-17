//! ChainQuest Idle - Client application

use chainquest_idle::run_game;
use env_logger;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    println!("Starting ChainQuest Idle - MVP Client");
    run_game();
}
