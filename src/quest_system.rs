//! Quest system implementation for ChainQuest Idle

use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use serde::{Deserialize, Serialize};
use rand::prelude::*;

/// Quest generation and management resource
#[derive(Resource, Debug)]
pub struct QuestManager {
    pub active_quests: Vec<Entity>,
    pub completed_quests: Vec<u32>,
    pub next_quest_id: u32,
    pub quest_timer: f32,
}

impl Default for QuestManager {
    fn default() -> Self {
        Self {
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
            next_quest_id: 1,
            quest_timer: 0.0,
        }
    }
}

/// Quest templates for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestTemplate {
    pub name_template: String,
    pub description_template: String,
    pub reward_resources: f32,
    pub completion_time: f32,
    pub difficulty: QuestDifficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestDifficulty {
    Easy,
    Medium,
    Hard,
    Epic,
}

impl QuestDifficulty {
    pub fn reward_multiplier(&self) -> f32 {
        match self {
            QuestDifficulty::Easy => 1.0,
            QuestDifficulty::Medium => 2.0,
            QuestDifficulty::Hard => 4.0,
            QuestDifficulty::Epic => 8.0,
        }
    }
}

/// Initialize quest system
pub fn setup_quest_system(mut commands: Commands) {
    commands.insert_resource(QuestManager::default());
    info!("Quest system initialized");
}

/// Generate new quests periodically
pub fn generate_quests(
    mut commands: Commands,
    mut quest_manager: ResMut<QuestManager>,
    time: Res<Time>,
    query: Query<&IdleProgress, With<Player>>,
) {
    quest_manager.quest_timer += time.delta_seconds();
    
    // Generate new quest every 30 seconds if less than 3 active
    if quest_manager.quest_timer >= 30.0 && quest_manager.active_quests.len() < 3 {
        if let Ok(player_progress) = query.get_single() {
            let quest_entity = spawn_quest(&mut commands, &mut quest_manager, player_progress.level);
            quest_manager.active_quests.push(quest_entity);
            quest_manager.quest_timer = 0.0;
        }
    }
}

/// Spawn a new quest entity
fn spawn_quest(commands: &mut Commands, quest_manager: &mut QuestManager, player_level: u32) -> Entity {
    let mut rng = rand::thread_rng();
    
    let templates = get_quest_templates();
    let template = templates.choose(&mut rng).unwrap();
    
    let difficulty = match player_level {
        1..=5 => QuestDifficulty::Easy,
        6..=15 => if rng.gen_bool(0.7) { QuestDifficulty::Easy } else { QuestDifficulty::Medium },
        16..=30 => match rng.gen_range(0..3) {
            0 => QuestDifficulty::Easy,
            1 => QuestDifficulty::Medium,
            _ => QuestDifficulty::Hard,
        },
        _ => match rng.gen_range(0..4) {
            0 => QuestDifficulty::Medium,
            1 => QuestDifficulty::Hard,
            2 => QuestDifficulty::Hard,
            _ => QuestDifficulty::Epic,
        }
    };
    
    let base_reward = template.reward_resources * difficulty.reward_multiplier();
    let level_multiplier = (player_level as f32).sqrt();
    let final_reward = base_reward * level_multiplier;
    
    let quest_id = quest_manager.next_quest_id;
    quest_manager.next_quest_id += 1;
    
    let sft_reward = if matches!(difficulty, QuestDifficulty::Hard | QuestDifficulty::Epic) {
        Some(SFTAttributes {
            quest_id,
            map_seed: rng.gen(),
            rarity: match difficulty {
                QuestDifficulty::Hard => if rng.gen_bool(0.8) { Rarity::Rare } else { Rarity::Epic },
                QuestDifficulty::Epic => if rng.gen_bool(0.6) { Rarity::Epic } else { Rarity::Legendary },
                _ => Rarity::Common,
            },
            power: rng.gen_range(10..100) * difficulty.reward_multiplier() as u32,
            metadata: format!("Quest {} Reward", quest_id),
        })
    } else {
        None
    };
    
    let quest = Quest {
        id: quest_id,
        name: template.name_template.replace("{level}", &player_level.to_string()),
        description: template.description_template.replace("{reward}", &final_reward.round().to_string()),
        completed: false,
        reward_resources: final_reward,
        reward_sft: sft_reward,
    };
    
    info!("Generated quest: {} (ID: {})", quest.name, quest.id);
    
    commands.spawn(quest).id()
}

/// Process quest completion
pub fn process_quest_completion(
    mut commands: Commands,
    mut quest_manager: ResMut<QuestManager>,
    mut player_query: Query<&mut IdleProgress, With<Player>>,
    mut quest_query: Query<(Entity, &mut Quest)>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        // Complete oldest active quest when Q is pressed
        if let Some(&quest_entity) = quest_manager.active_quests.first() {
            if let Ok((entity, mut quest)) = quest_query.get_mut(quest_entity) {
                if !quest.completed {
                    quest.completed = true;
                    quest_manager.completed_quests.push(quest.id);
                    
                    // Reward player
                    if let Ok(mut player_progress) = player_query.get_single_mut() {
                        player_progress.resources += quest.reward_resources;
                        info!("Quest completed! Gained {} resources. Quest: {}", quest.reward_resources, quest.name);
                        
                        // TODO: Trigger SFT minting if quest.reward_sft is Some
                        if let Some(ref sft_attributes) = quest.reward_sft {
                            info!("SFT reward earned: {:?}", sft_attributes);
                            // This will be connected to smart contract minting
                        }
                    }
                    
                    // Remove from active quests
                    quest_manager.active_quests.retain(|&e| e != entity);
                    commands.entity(entity).despawn();
                }
            }
        }
    }
    
    // Auto-complete quests after their completion time
    let current_time = time.elapsed_seconds();
    let mut completed_entities = Vec::new();
    
    for (entity, mut quest) in quest_query.iter_mut() {
        if !quest.completed && current_time >= quest.reward_resources / 10.0 { // Simple time-based completion
            quest.completed = true;
            completed_entities.push(entity);
        }
    }
    
    for entity in completed_entities {
        if let Ok((_, quest)) = quest_query.get(entity) {
            quest_manager.completed_quests.push(quest.id);
            quest_manager.active_quests.retain(|&e| e != entity);
            commands.entity(entity).despawn();
        }
    }
}

/// Get predefined quest templates
fn get_quest_templates() -> Vec<QuestTemplate> {
    vec![
        QuestTemplate {
            name_template: "Collect Ancient Crystals (Lv.{level})".to_string(),
            description_template: "Gather mystical crystals to earn {reward} resources".to_string(),
            reward_resources: 50.0,
            completion_time: 60.0,
            difficulty: QuestDifficulty::Easy,
        },
        QuestTemplate {
            name_template: "Defeat Shadow Beasts (Lv.{level})".to_string(),
            description_template: "Eliminate dangerous creatures for {reward} resources".to_string(),
            reward_resources: 100.0,
            completion_time: 120.0,
            difficulty: QuestDifficulty::Medium,
        },
        QuestTemplate {
            name_template: "Explore Lost Dungeons (Lv.{level})".to_string(),
            description_template: "Venture into forgotten realms for {reward} resources".to_string(),
            reward_resources: 200.0,
            completion_time: 300.0,
            difficulty: QuestDifficulty::Hard,
        },
        QuestTemplate {
            name_template: "Conquer Dragon's Lair (Lv.{level})".to_string(),
            description_template: "Face the ultimate challenge for {reward} resources".to_string(),
            reward_resources: 500.0,
            completion_time: 600.0,
            difficulty: QuestDifficulty::Epic,
        },
    ]
}