use std::net::{SocketAddr, UdpSocket};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;
use crate::config::MeshConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerNode {
    pub id: String,
    pub endpoint: SocketAddr,
    pub last_seen: u64,
}

pub struct PeerRegistry {
    peers: HashMap<String, PeerNode>,
    node_id: String,
    #[allow(dead_code)]
    listen_addr: SocketAddr,
    broadcast_port: u16,
}

impl PeerRegistry {
    pub fn new_from_config(config: &MeshConfig) -> Self {
        let mut registry = Self {
            peers: HashMap::new(),
            node_id: config.node_id.clone(),
            listen_addr: config.listen_addr,
            broadcast_port: config.broadcast_port,
        };

        for static_peer in &config.static_peers {
            registry.register_peer(static_peer.id.clone(), static_peer.endpoint);
        }

        registry
    }

    pub fn register_peer(&mut self, id: String, endpoint: SocketAddr) {
        let peer = PeerNode {
            id: id.clone(),
            endpoint,
            last_seen: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.peers.insert(id, peer);
    }

    pub fn get_active_peers(&self) -> Vec<&PeerNode> {
        self.peers.values().collect()
    }

    /// Starts the background UDP broadcast and listener loop
    pub fn start_udp_discovery(&self) -> Result<(), Box<dyn std::error::Error>> {
        let bind_addr = format!("0.0.0.0:{}", self.broadcast_port);
        let socket = UdpSocket::bind(&bind_addr)?;
        socket.set_broadcast(true)?;

        println!("[DISCOVERY] UDP broadcast listening on port {}", self.broadcast_port);

        let broadcast_port = self.broadcast_port;
        let node_id = self.node_id.clone();

        // Spawn background thread for periodic announcements
        thread::spawn(move || {
            let sender_socket = match UdpSocket::bind("0.0.0.0:0") {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("[DISCOVERY] Failed to bind sender socket: {}", e);
                    return;
                }
            };
            let _ = sender_socket.set_broadcast(true);

            let broadcast_target = format!("255.255.255.255:{}", broadcast_port);
            let message = format!("OMEGA_PING:{}", node_id);

            loop {
                let _ = sender_socket.send_to(message.as_bytes(), &broadcast_target);
                thread::sleep(Duration::from_secs(5));
            }
        });

        Ok(())
    }
}
