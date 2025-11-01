//! Security and anti-cheat systems for ChainQuest Idle

use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::RwLock;
use std::sync::Arc;

/// Security manager resource for anti-cheat protection
#[derive(Resource, Debug)]
pub struct SecurityManager {
    pub player_actions: Arc<RwLock<HashMap<u32, PlayerActionHistory>>>,
    pub validation_config: ValidationConfig,
}

#[derive(Debug, Clone)]
pub struct PlayerActionHistory {
    pub last_resource_collection: u64,
    pub last_quest_completion: u64,
    pub last_level_up: u64,
    pub actions_per_second: f32,
    pub suspicious_activity_count: u32,
}

#[derive(Debug, Clone)]
pub struct ValidationConfig {
    pub max_actions_per_second: f32,
    pub min_time_between_quests: u64, // seconds
    pub max_resource_gain_per_action: f32,
    pub max_level_jumps: u32,
    pub suspicious_threshold: u32,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_actions_per_second: 10.0,
            min_time_between_quests: 5, // 5 seconds minimum between quests
            max_resource_gain_per_action: 1000.0,
            max_level_jumps: 5, // Max 5 levels at once
            suspicious_threshold: 10,
        }
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self {
            player_actions: Arc::new(RwLock::new(HashMap::new())),
            validation_config: ValidationConfig::default(),
        }
    }
}

impl SecurityManager {
    /// Validate a resource collection action
    pub fn validate_resource_collection(
        &self, 
        player_id: u32, 
        amount: f32
    ) -> ValidationResult {
        let current_time = get_current_timestamp();
        let mut actions = self.player_actions.write();
        let player_history = actions.entry(player_id).or_insert_with(|| PlayerActionHistory {
            last_resource_collection: 0,
            last_quest_completion: 0,
            last_level_up: 0,
            actions_per_second: 0.0,
            suspicious_activity_count: 0,
        });
        
        // Check for excessive resource gain
        if amount > self.validation_config.max_resource_gain_per_action {
            player_history.suspicious_activity_count += 1;
            warn!("Player {} attempting excessive resource gain: {}", player_id, amount);
            return ValidationResult::Rejected("Excessive resource gain detected".to_string());
        }
        
        // Check action rate
        let time_since_last = current_time.saturating_sub(player_history.last_resource_collection);
        if time_since_last < 1 { // Less than 1 second
            player_history.actions_per_second += 1.0;
        } else {
            player_history.actions_per_second = 1.0;
        }
        
        if player_history.actions_per_second > self.validation_config.max_actions_per_second {
            player_history.suspicious_activity_count += 1;
            warn!("Player {} exceeding action rate limit: {} actions/sec", player_id, player_history.actions_per_second);
            return ValidationResult::RateLimited;
        }
        
        player_history.last_resource_collection = current_time;
        
        // Check suspicious activity threshold
        if player_history.suspicious_activity_count >= self.validation_config.suspicious_threshold {
            error!("Player {} flagged for suspicious activity", player_id);
            return ValidationResult::Flagged;
        }
        
        ValidationResult::Approved
    }
    
    /// Validate a quest completion
    pub fn validate_quest_completion(
        &self, 
        player_id: u32, 
        quest_id: u32
    ) -> ValidationResult {
        let current_time = get_current_timestamp();
        let mut actions = self.player_actions.write();
        let player_history = actions.entry(player_id).or_insert_with(|| PlayerActionHistory {
            last_resource_collection: 0,
            last_quest_completion: 0,
            last_level_up: 0,
            actions_per_second: 0.0,
            suspicious_activity_count: 0,
        });
        
        // Check minimum time between quests
        let time_since_last = current_time.saturating_sub(player_history.last_quest_completion);
        if time_since_last < self.validation_config.min_time_between_quests {
            player_history.suspicious_activity_count += 1;
            warn!("Player {} completing quests too quickly: {}s since last", player_id, time_since_last);
            return ValidationResult::Rejected("Quest completion too frequent".to_string());
        }
        
        player_history.last_quest_completion = current_time;
        info!("Quest {} completed by player {} validated", quest_id, player_id);
        
        ValidationResult::Approved
    }
    
    /// Validate level progression
    pub fn validate_level_up(
        &self, 
        player_id: u32, 
        old_level: u32, 
        new_level: u32
    ) -> ValidationResult {
        let level_jump = new_level.saturating_sub(old_level);
        
        if level_jump > self.validation_config.max_level_jumps {
            let mut actions = self.player_actions.write();
            if let Some(player_history) = actions.get_mut(&player_id) {
                player_history.suspicious_activity_count += 5; // Severe penalty
            }
            error!("Player {} suspicious level jump: {} -> {} (+{})", player_id, old_level, new_level, level_jump);
            return ValidationResult::Rejected("Suspicious level progression".to_string());
        }
        
        let mut actions = self.player_actions.write();
        if let Some(player_history) = actions.get_mut(&player_id) {
            player_history.last_level_up = get_current_timestamp();
        }
        
        ValidationResult::Approved
    }
    
    /// Get player security status
    pub fn get_player_status(&self, player_id: u32) -> Option<PlayerSecurityStatus> {
        let actions = self.player_actions.read();
        actions.get(&player_id).map(|history| {
            let is_flagged = history.suspicious_activity_count >= self.validation_config.suspicious_threshold;
            let is_rate_limited = history.actions_per_second > self.validation_config.max_actions_per_second;
            
            PlayerSecurityStatus {
                player_id,
                suspicious_activity_count: history.suspicious_activity_count,
                actions_per_second: history.actions_per_second,
                is_flagged,
                is_rate_limited,
            }
        })
    }
    
    /// Reset player security status (admin function)
    pub fn reset_player_security(&self, player_id: u32) {
        let mut actions = self.player_actions.write();
        if let Some(player_history) = actions.get_mut(&player_id) {
            player_history.suspicious_activity_count = 0;
            player_history.actions_per_second = 0.0;
            info!("Security status reset for player {}", player_id);
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValidationResult {
    Approved,
    RateLimited,
    Flagged,
    Rejected(String),
}

#[derive(Debug, Clone)]
pub struct PlayerSecurityStatus {
    pub player_id: u32,
    pub suspicious_activity_count: u32,
    pub actions_per_second: f32,
    pub is_flagged: bool,
    pub is_rate_limited: bool,
}

/// Get current timestamp in seconds
fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// System to initialize security manager
pub fn setup_security_manager(mut commands: Commands) {
    commands.insert_resource(SecurityManager::default());
    info!("Security manager initialized with anti-cheat protection");
}

/// System to periodically clean up old security data
pub fn security_cleanup(
    security_manager: Res<SecurityManager>,
    time: Res<Time>,
) {
    // Run cleanup every 5 minutes
    if time.elapsed_seconds() as u64 % 300 == 0 {
        let current_time = get_current_timestamp();
        let mut actions = security_manager.player_actions.write();
        
        // Remove entries older than 1 hour
        actions.retain(|_, history| {
            let last_activity = history.last_resource_collection
                .max(history.last_quest_completion)
                .max(history.last_level_up);
            
            current_time.saturating_sub(last_activity) < 3600 // 1 hour
        });
        
        info!("Security cleanup completed, {} active players tracked", actions.len());
    }
}

/// Input sanitization utilities
pub mod input_sanitization {
    use regex::Regex;
    use std::sync::LazyLock;
    
    static USERNAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-zA-Z0-9_]{3,20}$").unwrap()
    });
    
    static SAFE_STRING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-zA-Z0-9\s\-_.,!?]{1,100}$").unwrap()
    });
    
    /// Sanitize and validate username input
    pub fn sanitize_username(username: &str) -> Result<String, String> {
        let trimmed = username.trim();
        
        if trimmed.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        
        if trimmed.len() < 3 {
            return Err("Username must be at least 3 characters".to_string());
        }
        
        if trimmed.len() > 20 {
            return Err("Username cannot exceed 20 characters".to_string());
        }
        
        if !USERNAME_REGEX.is_match(trimmed) {
            return Err("Username contains invalid characters".to_string());
        }
        
        Ok(trimmed.to_string())
    }
    
    /// Sanitize general text input (descriptions, messages)
    pub fn sanitize_text_input(text: &str) -> Result<String, String> {
        let trimmed = text.trim();
        
        if trimmed.is_empty() {
            return Err("Text cannot be empty".to_string());
        }
        
        if trimmed.len() > 100 {
            return Err("Text cannot exceed 100 characters".to_string());
        }
        
        if !SAFE_STRING_REGEX.is_match(trimmed) {
            return Err("Text contains invalid characters".to_string());
        }
        
        Ok(trimmed.to_string())
    }
    
    /// Validate numeric inputs
    pub fn validate_numeric_input(value: f32, min: f32, max: f32, name: &str) -> Result<f32, String> {
        if value.is_nan() || value.is_infinite() {
            return Err(format!("{} must be a valid number", name));
        }
        
        if value < min {
            return Err(format!("{} cannot be less than {}", name, min));
        }
        
        if value > max {
            return Err(format!("{} cannot exceed {}", name, max));
        }
        
        Ok(value)
    }
}