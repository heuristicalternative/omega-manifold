use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub struct P2PMeshNode {
    pub bind_addr: String,
    pub baseline_version: String,
    pub peers: Arc<Mutex<Vec<String>>>,
}

impl P2PMeshNode {
    pub fn new(bind_addr: &str, baseline_version: &str) -> Self {
        Self {
            bind_addr: bind_addr.to_string(),
            baseline_version: baseline_version.to_string(),
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn start_listener(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.bind_addr).await?;
        println!("[P2P Mesh] TCP listener active on {}", self.bind_addr);
        
        let peers = Arc::clone(&self.peers);
        tokio::spawn(async move {
            while let Ok((_stream, addr)) = listener.accept().await {
                println!("[P2P Mesh] Accepted incoming connection from {}", addr);
                let mut p = peers.lock().await;
                p.push(addr.to_string());
            }
        });
        
        Ok(())
    }

    pub fn connect_peer(&mut self, addr: &str) {
        println!("[P2P Mesh] Connecting to peer at {}...", addr);
    }

    pub fn broadcast_gossip_frame(&self, frame: &str) {
        println!("[P2P Mesh] Broadcasting gossip frame: {}", frame);
    }
}
