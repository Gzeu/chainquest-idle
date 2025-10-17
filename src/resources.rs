//! Game resources and global state

use bevy::prelude::*;
use rusqlite::{Connection, Result};
use serde_json;
use crate::components::IdleProgress;
use std::sync::{Arc, Mutex};

/// Global game state
#[derive(Resource, Default)]
pub struct GameState {
    pub current_map_seed: i64,
    pub multiplayer_connected: bool,
    pub blockchain_connected: bool,
    pub total_players: usize,
}

/// Database connection resource
#[derive(Resource)]
pub struct DatabaseConnection {
    conn: Arc<Mutex<Connection>>,
}

impl DatabaseConnection {
    /// Create new database connection
    pub fn new() -> Self {
        let conn = Connection::open("chainquest.db")
            .expect("Failed to open database");
        
        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS progress (
                id INTEGER PRIMARY KEY,
                resources REAL NOT NULL,
                experience REAL NOT NULL,
                level INTEGER NOT NULL,
                last_update REAL NOT NULL
            )",
            [],
        ).expect("Failed to create progress table");
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS maps (
                id INTEGER PRIMARY KEY,
                seed INTEGER NOT NULL,
                grid TEXT NOT NULL,
                created_at REAL NOT NULL
            )",
            [],
        ).expect("Failed to create maps table");
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sft_assets (
                id INTEGER PRIMARY KEY,
                token_id TEXT NOT NULL,
                attributes TEXT NOT NULL,
                staked INTEGER NOT NULL DEFAULT 0
            )",
            [],
        ).expect("Failed to create sft_assets table");
        
        info!("Database initialized successfully");
        
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
    
    /// Save player progress
    pub fn save_progress(&self, progress: &IdleProgress) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO progress (id, resources, experience, level, last_update) 
             VALUES (1, ?1, ?2, ?3, ?4)",
            [progress.resources, progress.experience, progress.level as f32, progress.last_update],
        )?;
        Ok(())
    }
    
    /// Load player progress
    pub fn load_progress(&self) -> Result<IdleProgress> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT resources, experience, level, last_update FROM progress WHERE id = 1"
        )?;
        
        let progress = stmt.query_row([], |row| {
            Ok(IdleProgress {
                resources: row.get(0)?,
                experience: row.get(1)?,
                level: row.get::<_, f32>(2)? as u32,
                last_update: row.get(3)?,
            })
        })?;
        
        Ok(progress)
    }
    
    /// Save generated map
    pub fn save_map(&self, seed: i64, grid: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
            
        conn.execute(
            "INSERT INTO maps (seed, grid, created_at) VALUES (?1, ?2, ?3)",
            [seed.to_string(), grid.to_string(), timestamp.to_string()],
        )?;
        Ok(())
    }
    
    /// Load map by seed
    pub fn load_map(&self, seed: i64) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT grid FROM maps WHERE seed = ?1")?;
        let grid: String = stmt.query_row([seed], |row| row.get(0))?;
        Ok(grid)
    }
}

/// Multiplayer connection state
#[derive(Resource, Default)]
pub struct MultiplayerState {
    pub server_address: String,
    pub player_id: u32,
    pub connected_peers: Vec<u32>,
    pub is_host: bool,
}

/// Blockchain connection state
#[derive(Resource, Default)]
pub struct BlockchainState {
    pub wallet_address: String,
    pub testnet_connected: bool,
    pub pending_transactions: Vec<String>,
    pub sft_balance: u32,
}

/// AI generation state
#[derive(Resource, Default)]
pub struct AIState {
    pub model_loaded: bool,
    pub generation_cache: std::collections::HashMap<i64, String>,
    pub last_generation_time: f64,
}