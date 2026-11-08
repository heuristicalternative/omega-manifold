use omega_manifold::sync_manager::SyncManager;
use std::net::TcpListener;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr)?;
    println!("[SYNC_NODE] Omega Manifold synchronization listener active on {}", addr);

    let manager = SyncManager::new("manifold_state.db");
    manager.verify_state_integrity()?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[SYNC_NODE] Incoming peer sync connection accepted.");
                if let Err(e) = manager.handle_client(stream) {
                    eprintln!("[SYNC_NODE] Error handling sync client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("[SYNC_NODE] Connection error: {}", e);
            }
        }
    }

    Ok(())
}
