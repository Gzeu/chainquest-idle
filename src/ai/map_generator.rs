//! AI-powered map generation using torch-rs

use bevy::prelude::*;
use tch::{nn, Device, Tensor, CModule};
use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;
use crate::components::{TileType, MapTile};
use std::collections::HashMap;

/// AI Map Generator resource
#[derive(Resource, Debug)]
pub struct MapGenerator {
    pub device: Device,
    pub model: Option<CModule>,
    pub cache: HashMap<i64, Vec<Vec<i32>>>,
    pub generation_stats: GenerationStats,
}

#[derive(Debug, Default)]
pub struct GenerationStats {
    pub maps_generated: u32,
    pub cache_hits: u32,
    pub average_generation_time_ms: f32,
}

impl Default for MapGenerator {
    fn default() -> Self {
        let device = if tch::Cuda::is_available() {
            info!("CUDA available, using GPU for map generation");
            Device::Cuda(0)
        } else {
            info!("CUDA not available, using CPU for map generation");
            Device::Cpu
        };
        
        Self {
            device,
            model: None,
            cache: HashMap::new(),
            generation_stats: GenerationStats::default(),
        }
    }
}

impl MapGenerator {
    /// Initialize the AI model for map generation
    pub fn initialize_model(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Try to load a pre-trained model, fallback to procedural generation
        match self.load_pretrained_model() {
            Ok(model) => {
                self.model = Some(model);
                info!("AI model loaded successfully for map generation");
            }
            Err(e) => {
                warn!("Failed to load AI model, using procedural generation: {}", e);
                // Model remains None, will use fallback generation
            }
        }
        Ok(())
    }
    
    /// Load a pre-trained PyTorch model
    fn load_pretrained_model(&self) -> Result<CModule, Box<dyn std::error::Error>> {
        // This would load an actual trained model in production
        // For now, we'll create a simple neural network as a placeholder
        let vs = nn::VarStore::new(self.device);
        let model = nn::seq()
            .add(nn::linear(&vs.root(), 64, 128, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::linear(&vs.root(), 128, 256, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::linear(&vs.root(), 256, 16 * 16 * 4, Default::default()))
            .add_fn(|x| x.softmax(-1, tch::Kind::Float));
        
        // Convert to CModule for inference
        let dummy_input = Tensor::randn(&[1, 64], (tch::Kind::Float, self.device));
        let traced = model.forward(&dummy_input);
        
        // In a real scenario, you would load from a .pt file:
        // CModule::load("models/map_generator.pt")
        Err("No pre-trained model available".into())
    }
    
    /// Generate a 16x16 map using AI or procedural fallback
    pub fn generate_map(&mut self, seed: i64) -> Vec<Vec<i32>> {
        let start_time = std::time::Instant::now();
        
        // Check cache first
        if let Some(cached_map) = self.cache.get(&seed) {
            self.generation_stats.cache_hits += 1;
            return cached_map.clone();
        }
        
        let map = if let Some(ref model) = self.model {
            self.generate_with_ai(model, seed)
        } else {
            self.generate_procedural(seed)
        };
        
        let generation_time = start_time.elapsed().as_millis() as f32;
        self.update_stats(generation_time);
        
        // Cache the result
        self.cache.insert(seed, map.clone());
        
        // Limit cache size to prevent memory issues
        if self.cache.len() > 100 {
            let oldest_key = *self.cache.keys().next().unwrap();
            self.cache.remove(&oldest_key);
        }
        
        map
    }
    
    /// Generate map using the AI model
    fn generate_with_ai(&self, model: &CModule, seed: i64) -> Vec<Vec<i32>> {
        // Prepare seed as tensor input
        let seed_tensor = Tensor::of_slice(&[seed as f32])
            .to_device(self.device)
            .unsqueeze(0)
            .expand(&[1, 64], true); // Expand to expected input size
        
        // Run inference
        let output = tch::no_grad(|| {
            model.forward_ts(&[seed_tensor]).unwrap()
        });
        
        // Convert output tensor to 16x16 grid
        self.tensor_to_grid(output, seed)
    }
    
    /// Generate map using procedural method
    fn generate_procedural(&self, seed: i64) -> Vec<Vec<i32>> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
        let mut grid = vec![vec![0; 16]; 16];
        
        // Enhanced procedural generation with biomes and structures
        let biome = rng.gen_range(0..4); // 0: Forest, 1: Desert, 2: Mountains, 3: Swamp
        
        for x in 0..16 {
            for y in 0..16 {
                let distance_from_center = ((x as f32 - 8.0).powi(2) + (y as f32 - 8.0).powi(2)).sqrt();
                let noise = (x as f32 * 0.3).sin() * (y as f32 * 0.3).cos() * 0.5;
                
                let base_tile = match biome {
                    0 => { // Forest
                        if rng.gen_bool(0.3) { 1 } else { 0 } // Resources in forest
                    }
                    1 => { // Desert
                        if rng.gen_bool(0.1) { 1 } else if rng.gen_bool(0.15) { 2 } else { 0 }
                    }
                    2 => { // Mountains
                        if distance_from_center > 6.0 && rng.gen_bool(0.4) { 1 } else { 0 }
                    }
                    _ => { // Swamp
                        if rng.gen_bool(0.2) { 2 } else { 0 } // More enemies
                    }
                };
                
                // Add some structure
                let tile = if distance_from_center < 2.0 && rng.gen_bool(0.1) {
                    3 // Quest location near center
                } else if x == 0 || x == 15 || y == 0 || y == 15 {
                    if rng.gen_bool(0.05) { 4 } else { base_tile } // Rare portals on edges
                } else {
                    base_tile
                };
                
                grid[x][y] = tile;
            }
        }
        
        // Ensure at least one quest and one resource node
        if !grid.iter().any(|row| row.contains(&3)) {
            grid[8][8] = 3; // Quest in center
        }
        if !grid.iter().any(|row| row.contains(&1)) {
            grid[rng.gen_range(1..15)][rng.gen_range(1..15)] = 1; // Random resource
        }
        
        grid
    }
    
    /// Convert AI tensor output to 16x16 grid
    fn tensor_to_grid(&self, output: Tensor, seed: i64) -> Vec<Vec<i32>> {
        let output_data: Vec<f32> = output.reshape(&[16, 16, 4]).into();
        let mut grid = vec![vec![0; 16]; 16];
        
        for x in 0..16 {
            for y in 0..16 {
                // Find the tile type with highest probability
                let base_idx = (x * 16 + y) * 4;
                let mut max_prob = 0.0;
                let mut best_tile = 0;
                
                for tile_type in 0..4 {
                    if base_idx + tile_type < output_data.len() {
                        let prob = output_data[base_idx + tile_type];
                        if prob > max_prob {
                            max_prob = prob;
                            best_tile = tile_type;
                        }
                    }
                }
                
                grid[x][y] = best_tile as i32;
            }
        }
        
        // Post-process to ensure valid map (similar to procedural)
        self.ensure_valid_map(&mut grid, seed);
        grid
    }
    
    /// Ensure the generated map has required elements
    fn ensure_valid_map(&self, grid: &mut Vec<Vec<i32>>, seed: i64) {
        let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
        
        // Ensure at least one quest
        if !grid.iter().any(|row| row.contains(&3)) {
            let x = rng.gen_range(4..12);
            let y = rng.gen_range(4..12);
            grid[x][y] = 3;
        }
        
        // Ensure at least one resource
        if !grid.iter().any(|row| row.contains(&1)) {
            let x = rng.gen_range(1..15);
            let y = rng.gen_range(1..15);
            grid[x][y] = 1;
        }
    }
    
    /// Update generation statistics
    fn update_stats(&mut self, generation_time_ms: f32) {
        self.generation_stats.maps_generated += 1;
        let count = self.generation_stats.maps_generated as f32;
        self.generation_stats.average_generation_time_ms = 
            (self.generation_stats.average_generation_time_ms * (count - 1.0) + generation_time_ms) / count;
    }
    
    /// Get generation statistics
    pub fn get_stats(&self) -> &GenerationStats {
        &self.generation_stats
    }
}

/// Convert internal tile representation to TileType
pub fn int_to_tile_type(tile_int: i32) -> TileType {
    match tile_int {
        0 => TileType::Empty,
        1 => TileType::Resource,
        2 => TileType::Enemy,
        3 => TileType::Quest,
        4 => TileType::Portal,
        _ => TileType::Empty,
    }
}

/// System to initialize AI map generation
pub fn setup_ai_map_generator(mut commands: Commands) {
    let mut generator = MapGenerator::default();
    
    if let Err(e) = generator.initialize_model() {
        warn!("Failed to initialize AI model: {}", e);
    }
    
    commands.insert_resource(generator);
    info!("AI Map Generator initialized");
}

/// System to handle map generation requests
pub fn handle_map_generation(
    mut map_generator: ResMut<MapGenerator>,
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        let seed = rand::random::<i64>();
        let map_data = map_generator.generate_map(seed);
        
        info!("Generated new map with seed: {}", seed);
        info!("Map generation stats: {:?}", map_generator.get_stats());
        
        // Spawn map tiles as entities
        for (x, row) in map_data.iter().enumerate() {
            for (y, &tile_value) in row.iter().enumerate() {
                let tile_type = int_to_tile_type(tile_value);
                let tile = MapTile {
                    tile_type,
                    grid_x: x as i32,
                    grid_y: y as i32,
                };
                
                commands.spawn(tile);
            }
        }
        
        info!("Spawned {} map tiles", map_data.len() * map_data[0].len());
    }
}