use omega_manifold::sync_manager::SyncManager;
use std::net::TcpStream;

#[cfg(unix)]
use std::os::unix::io::FromRawFd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = SyncManager::new("manifold_state.db");
    manager.verify_state_integrity()?;

    let stream = if std::env::var("INVOCATION_ID").is_ok() {
        println!("[SYNC_NODE] Running via systemd socket activation (Accept=yes).");
        #[cfg(unix)]
        unsafe { TcpStream::from_raw_fd(0) }
        #[cfg(not(unix))]
        {
            return Err("Socket activation only supported on Unix".into());
        }
    } else {
        eprintln!("[SYNC_NODE] Error: Expected systemd socket activation stream.");
        return Err("Not invoked via systemd socket activation".into());
    };

    // Handle client safely, ignoring broken pipe or EOF disconnects from probes like netcat
    if let Err(e) = manager.handle_client(stream) {
        eprintln!("[SYNC_NODE] Client connection finished or dropped: {}", e);
    }

    Ok(())
}
