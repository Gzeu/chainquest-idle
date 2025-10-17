use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use enet::{Address, Event, Host, Packet, PacketMode, Peer};
use std::net::Ipv4Addr;
use std::time::Duration;
use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Resource, Default, Clone)]
pub struct NetConfig { pub host: String, pub port: u16 }

#[derive(Resource, Default, Clone)]
pub struct NetState { pub connected: bool, pub last_rtt: u32, pub last_msg: String }

#[derive(Resource)]
pub struct NetClient {
    pub host: Arc<Mutex<Host>>,
    pub peer: Arc<Mutex<Option<Peer>>>,
}

impl NetClient {
    pub fn new() -> Self {
        let _enet = enet::initialize().expect("ENet init");
        let host = Host::new(None, 1, 2, 0, 0).expect("client host");
        Self { host: Arc::new(Mutex::new(host)), peer: Arc::new(Mutex::new(None)) }
    }
}

pub fn net_setup(mut commands: Commands) {
    commands.insert_resource(NetClient::new());
    commands.insert_resource(NetConfig { host: "127.0.0.1".into(), port: 8080 });
    commands.insert_resource(NetState::default());
}

pub fn net_connect(client: Res<NetClient>, cfg: Res<NetConfig>, mut state: ResMut<NetState>) {
    if state.connected { return; }
    let addr = Address::new(Ipv4Addr::new(127,0,0,1), cfg.port);
    if let Ok(p) = client.host.lock().connect(&addr, 2, 0) {
        *client.peer.lock() = Some(p);
    }
}

pub fn net_service(client: Res<NetClient>, mut state: ResMut<NetState>) {
    if let Some(event) = client.host.lock().service(Duration::from_millis(5)).unwrap() {
        match event {
            Event::Connect(_peer) => { state.connected = true; state.last_msg = "Connected".into(); }
            Event::Disconnect(_peer, _reason) => { state.connected = false; state.last_msg = "Disconnected".into(); }
            Event::Receive{packet, ..} => {
                state.last_msg = format!("Echo {} bytes", packet.data().len());
            }
            _ => {}
        }
    }
}

pub fn net_ping(client: Res<NetClient>, state: Res<NetState>) {
    if !state.connected { return; }
    if let Some(peer) = client.peer.lock().as_ref() {
        let _ = peer.send_packet(Packet::new(b"ping", PacketMode::ReliableSequenced).unwrap(), 0);
    }
}
