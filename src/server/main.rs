use enet::{self, *};
use std::time::Duration;
use std::net::Ipv4Addr;
use log::*;
use env_logger;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting ENet server on 0.0.0.0:8080");
    let _enet = enet::initialize().expect("Failed to init ENet");

    let address = Address::new(Ipv4Addr::UNSPECIFIED, 8080);
    let mut server = Host::new(
        Some(&address),
        8,   // max clients
        2,   // channels
        0,   // in bandwidth
        0,   // out bandwidth
    ).expect("failed to create server host");

    loop {
        if let Some(event) = server.service(Duration::from_millis(50)).unwrap() {
            match event {
                Event::Connect(peer) => {
                    info!("Client connected: {:?}", peer.address());
                }
                Event::Disconnect(peer, reason) => {
                    info!("Client disconnected: {:?} reason={:?}", peer.address(), reason);
                }
                Event::Receive{packet, channel_id, peer} => {
                    let data = packet.data();
                    info!("Received {} bytes on ch {} from {:?}", data.len(), channel_id, peer.address());
                    // Echo back for MVP
                    let _ = peer.send_packet(Packet::new(data, PacketMode::ReliableSequenced).unwrap(), channel_id);
                }
                _ => {}
            }
        }
    }
}
