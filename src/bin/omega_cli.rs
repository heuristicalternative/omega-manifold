use omega_manifold::sync_manager::SyncManager;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let db_path = "manifold_state.db";
    let manager = SyncManager::new(db_path);

    if args.len() < 2 {
        println!("Omega Manifold CLI Manager");
        println!("Usage:");
        println!("  cargo run --bin omega_cli -- status");
        println!("  cargo run --bin omega_cli -- events [limit]");
        return Ok(());
    }

    match args[1].as_str() {
        "status" => {
            println!("[OMEGA_CLI] Inspecting local node database state...");
            let integrity = manager.verify_state_integrity()?;
            println!("[OMEGA_CLI] Database path: {}", db_path);
            println!("[OMEGA_CLI] State integrity verified: {}", integrity);
        }
        "events" => {
            let limit: i64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);
            println!("[OMEGA_CLI] Fetching up to {} pending telemetry events...", limit);
            let events = manager.fetch_pending_events(limit)?;
            let json = serde_json::to_string_pretty(&events)?;
            println!("{}", json);
        }
        _ => {
            eprintln!("[OMEGA_CLI] Unknown command: {}", args[1]);
        }
    }

    Ok(())
}
