//! Enhanced network module with rate limiting and compression

use bevy::prelude::*;
use enet::{Event, Host, Packet, PacketMode};
use flate2::Compression;
use flate2::write::{GzEncoder, GzDecoder};
use std::io::Write;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};

/// Network manager resource with rate limiting
#[derive(Resource, Debug)]
pub struct NetworkManager {
    pub host: Option<Host<u32>>,
    pub peer_rate_limits: HashMap<u32, RateLimit>,
    pub compression_enabled: bool,
    pub stats: NetworkStats,
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub packets_sent: u32,
    pub last_reset: Instant,
    pub max_packets_per_second: u32,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub packets_sent: u64,
    pub packets_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub compression_ratio: f32,
    pub rate_limit_violations: u32,
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self {
            host: None,
            peer_rate_limits: HashMap::new(),
            compression_enabled: true,
            stats: NetworkStats::default(),
        }
    }
}

impl NetworkManager {
    /// Initialize network manager with rate limiting
    pub fn initialize(&mut self, max_connections: usize, port: u16) -> Result<(), String> {
        let address = enet::Address::new_any(port);
        
        match Host::new(Some(address), max_connections, 2, 0, 0) {
            Some(host) => {
                self.host = Some(host);
                info!("Network manager initialized on port {} with {} max connections", port, max_connections);
                Ok(())
            }
            None => Err("Failed to create ENet host".to_string())
        }
    }
    
    /// Send packet with rate limiting and compression
    pub fn send_packet(&mut self, peer_id: u32, data: &[u8], reliable: bool) -> Result<(), String> {
        // Check rate limit
        if !self.check_rate_limit(peer_id) {
            self.stats.rate_limit_violations += 1;
            return Err("Rate limit exceeded".to_string());
        }
        
        let processed_data = if self.compression_enabled && data.len() > 100 {
            self.compress_data(data)?
        } else {
            data.to_vec()
        };
        
        if let Some(ref mut host) = self.host {
            let packet_mode = if reliable {
                PacketMode::ReliableSequenced
            } else {
                PacketMode::UnreliableSequenced
            };
            
            match Packet::new(&processed_data, packet_mode) {
                Some(packet) => {
                    if let Some(peer) = host.peer(peer_id) {
                        peer.send_packet(packet, 0);
                        
                        // Update stats
                        self.stats.packets_sent += 1;
                        self.stats.bytes_sent += processed_data.len() as u64;
                        
                        if self.compression_enabled && data.len() > 100 {
                            self.stats.compression_ratio = (processed_data.len() as f32) / (data.len() as f32);
                        }
                        
                        Ok(())
                    } else {
                        Err("Peer not found".to_string())
                    }
                }
                None => Err("Failed to create packet".to_string())
            }
        } else {
            Err("Network not initialized".to_string())
        }
    }
    
    /// Process network events with decompression
    pub fn process_events(&mut self) -> Vec<NetworkEvent> {
        let mut events = Vec::new();
        
        if let Some(ref mut host) = self.host {
            while let Some(event) = host.service(Duration::from_millis(0)) {
                match event {
                    Event::Connect(peer) => {
                        let peer_id = peer.data();
                        info!("Peer {} connected", peer_id);
                        
                        // Initialize rate limit for new peer
                        self.peer_rate_limits.insert(peer_id, RateLimit {
                            packets_sent: 0,
                            last_reset: Instant::now(),
                            max_packets_per_second: 10, // Default 10 packets/sec
                        });
                        
                        events.push(NetworkEvent::PeerConnected(peer_id));
                    }
                    Event::Disconnect(peer, _) => {
                        let peer_id = peer.data();
                        info!("Peer {} disconnected", peer_id);
                        
                        // Clean up rate limit tracking
                        self.peer_rate_limits.remove(&peer_id);
                        
                        events.push(NetworkEvent::PeerDisconnected(peer_id));
                    }
                    Event::Receive { sender, data, .. } => {
                        let peer_id = sender.data();
                        
                        // Update stats
                        self.stats.packets_received += 1;
                        self.stats.bytes_received += data.len() as u64;
                        
                        // Decompress if needed
                        let processed_data = if self.compression_enabled && data.len() > 4 {
                            // Check if data is compressed (simple heuristic)
                            if data[0] == 0x1f && data[1] == 0x8b {
                                match self.decompress_data(&data) {
                                    Ok(decompressed) => decompressed,
                                    Err(e) => {
                                        warn!("Failed to decompress data from peer {}: {}", peer_id, e);
                                        data
                                    }
                                }
                            } else {
                                data
                            }
                        } else {
                            data
                        };
                        
                        events.push(NetworkEvent::DataReceived {
                            peer_id,
                            data: processed_data,
                        });
                    }
                }
            }
        }
        
        events
    }
    
    /// Check and update rate limit for peer
    fn check_rate_limit(&mut self, peer_id: u32) -> bool {
        let now = Instant::now();
        
        if let Some(rate_limit) = self.peer_rate_limits.get_mut(&peer_id) {
            // Reset counter if more than 1 second has passed
            if now.duration_since(rate_limit.last_reset) >= Duration::from_secs(1) {
                rate_limit.packets_sent = 0;
                rate_limit.last_reset = now;
            }
            
            if rate_limit.packets_sent >= rate_limit.max_packets_per_second {
                warn!("Rate limit exceeded for peer {}: {} packets/sec", peer_id, rate_limit.packets_sent);
                return false;
            }
            
            rate_limit.packets_sent += 1;
            true
        } else {
            // No rate limit tracking for this peer yet
            true
        }
    }
    
    /// Compress data using gzip
    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(data).map_err(|e| format!("Compression write error: {}", e))?;
        encoder.finish().map_err(|e| format!("Compression finish error: {}", e))
    }
    
    /// Decompress data using gzip
    fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut decoder = GzDecoder::new(Vec::new());
        decoder.write_all(data).map_err(|e| format!("Decompression write error: {}", e))?;
        decoder.finish().map_err(|e| format!("Decompression finish error: {}", e))
    }
    
    /// Get network statistics
    pub fn get_stats(&self) -> &NetworkStats {
        &self.stats
    }
    
    /// Set rate limit for specific peer
    pub fn set_peer_rate_limit(&mut self, peer_id: u32, max_packets_per_second: u32) {
        if let Some(rate_limit) = self.peer_rate_limits.get_mut(&peer_id) {
            rate_limit.max_packets_per_second = max_packets_per_second;
            info!("Updated rate limit for peer {} to {} packets/sec", peer_id, max_packets_per_second);
        }
    }
    
    /// Broadcast message to all connected peers
    pub fn broadcast(&mut self, data: &[u8], reliable: bool) -> Result<(), String> {
        let peer_ids: Vec<u32> = self.peer_rate_limits.keys().cloned().collect();
        
        for peer_id in peer_ids {
            if let Err(e) = self.send_packet(peer_id, data, reliable) {
                warn!("Failed to send broadcast to peer {}: {}", peer_id, e);
            }
        }
        
        Ok(())
    }
}

/// Network events
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    PeerConnected(u32),
    PeerDisconnected(u32),
    DataReceived { peer_id: u32, data: Vec<u8> },
}

/// Game message types for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMessage {
    PlayerJoin { username: String },
    PlayerLeave { player_id: u32 },
    ResourceUpdate { player_id: u32, resources: f32 },
    QuestComplete { player_id: u32, quest_id: u32 },
    MapGenerate { seed: i64 },
    Chat { player_id: u32, message: String },
    Ping,
    Pong,
}

impl GameMessage {
    /// Serialize message to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(self).map_err(|e| format!("Serialization error: {}", e))
    }
    
    /// Deserialize message from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(data).map_err(|e| format!("Deserialization error: {}", e))
    }
}

/// System to initialize network manager
pub fn setup_network_manager(mut commands: Commands) {
    let mut network_manager = NetworkManager::default();
    
    // Initialize server on port 8080
    if let Err(e) = network_manager.initialize(4, 8080) {
        error!("Failed to initialize network manager: {}", e);
    } else {
        info!("Network manager initialized successfully");
    }
    
    commands.insert_resource(network_manager);
}

/// System to process network events
pub fn process_network_events(
    mut network_manager: ResMut<NetworkManager>,
    mut commands: Commands,
) {
    let events = network_manager.process_events();
    
    for event in events {
        match event {
            NetworkEvent::PeerConnected(peer_id) => {
                // Spawn network player entity
                commands.spawn(crate::components::NetworkPlayer {
                    peer_id,
                    username: format!("Player_{}", peer_id),
                    connected: true,
                });
            }
            NetworkEvent::PeerDisconnected(peer_id) => {
                // Find and despawn network player entity
                // This would require a proper query system in a real implementation
                info!("Cleaning up resources for disconnected peer {}", peer_id);
            }
            NetworkEvent::DataReceived { peer_id, data } => {
                // Process game message
                match GameMessage::from_bytes(&data) {
                    Ok(message) => {
                        info!("Received message from peer {}: {:?}", peer_id, message);
                        // Handle specific message types here
                    }
                    Err(e) => {
                        warn!("Failed to parse message from peer {}: {}", peer_id, e);
                    }
                }
            }
        }
    }
}

/// System to send periodic network statistics
pub fn network_statistics(
    network_manager: Res<NetworkManager>,
    time: Res<Time>,
) {
    // Log statistics every 30 seconds
    if time.elapsed_seconds() as u64 % 30 == 0 {
        let stats = network_manager.get_stats();
        info!("Network Stats: Sent: {} packets/{} bytes, Received: {} packets/{} bytes, Compression: {:.2}, Rate violations: {}",
            stats.packets_sent, stats.bytes_sent,
            stats.packets_received, stats.bytes_received,
            stats.compression_ratio,
            stats.rate_limit_violations
        );
    }
}