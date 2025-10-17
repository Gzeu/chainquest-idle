//! ChainQuest Idle - Client application

use chainquest_idle::run_game;
use env_logger;

fn main() {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    println!("Starting ChainQuest Idle - MVP Client");
    println!("Controls:");
    println!("  SPACE - Collect resources manually");
    println!("  Q - Activate quest system");
    println!("  M - Generate AI map");
    println!("  ESC - Exit game");
    
    // Run the game
    run_game();
}