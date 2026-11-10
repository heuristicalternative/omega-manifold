use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct MeshConfig {
    pub node_id: String,
    pub listen_addr: SocketAddr,
    pub static_peers: Vec<StaticPeer>,
    pub enable_udp_broadcast: bool,
    pub broadcast_port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticPeer {
    pub id: String,
    pub endpoint: SocketAddr,
}

impl MeshConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: MeshConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
