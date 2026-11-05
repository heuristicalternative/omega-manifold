#![allow(dead_code)]
// Radicle peer-to-peer patch propagation and seed node synchronization.

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RadicleSyncDaemon {
    pub repo_id: String,
    pub local_head: String,
    pub patch_ids: Vec<String>,
    pub seed_nodes: Vec<String>,
    pub peer_heads: HashMap<String, String>,
    pub timeout_ms: u64,
    pub timed_out_peers: Vec<String>,
}

impl RadicleSyncDaemon {
    pub fn new(repo_id: &str, local_head: &str) -> Self {
        Self {
            repo_id: repo_id.to_string(),
            local_head: local_head.to_string(),
            patch_ids: Vec::new(),
            seed_nodes: Vec::new(),
            peer_heads: HashMap::new(),
            timeout_ms: 5000,
            timed_out_peers: Vec::new(),
        }
    }

    pub fn add_seed_node(&mut self, endpoint: &str) {
        if !self.seed_nodes.contains(&endpoint.to_string()) {
            self.seed_nodes.push(endpoint.to_string());
        }
    }

    pub fn register_patch(&mut self, patch_id: &str) {
        if !self.patch_ids.contains(&patch_id.to_string()) {
            self.patch_ids.push(patch_id.to_string());
        }
    }

    pub fn broadcast_patch(&self, patch_id: &str) -> Result<(), String> {
        if patch_id.trim().is_empty() {
            return Err("patch id must not be empty".to_string());
        }
        if self.seed_nodes.is_empty() {
            return Err("no seed nodes available for broadcast".to_string());
        }
        // Simulated broadcast: each seed node receives the patch announcement.
        for seed in &self.seed_nodes {
            println!(
                "[RADICLE_SYNC] Broadcasting patch {} to seed {}",
                patch_id, seed
            );
        }
        Ok(())
    }

    pub fn reconcile_remote_heads(&mut self) -> Result<usize, String> {
        if self.seed_nodes.is_empty() {
            return Err("no seed nodes available for reconciliation".to_string());
        }

        let mut reconciled = 0usize;
        for seed in self.seed_nodes.clone() {
            // Simulate peer reachability: seeds containing "timeout" are unreachable.
            if seed.contains("timeout") {
                self.timed_out_peers.push(seed.clone());
                continue;
            }
            self.peer_heads.insert(seed.clone(), self.local_head.clone());
            reconciled += 1;
        }
        Ok(reconciled)
    }

    pub fn start_background_loop(&self) {
        let repo_id = self.repo_id.clone();
        thread::spawn(move || {
            // Minimal background loop placeholder.
            thread::sleep(Duration::from_millis(10));
            println!("[RADICLE_SYNC] Background sync loop tick for {}", repo_id);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broadcast_patch_success() {
        let mut daemon = RadicleSyncDaemon::new("rad:xyz", "abc123");
        daemon.add_seed_node("seed.radicle.dev:8776");
        assert!(daemon.broadcast_patch("patch-1").is_ok());
    }

    #[test]
    fn test_broadcast_patch_no_seeds() {
        let daemon = RadicleSyncDaemon::new("rad:xyz", "abc123");
        assert!(daemon.broadcast_patch("patch-1").is_err());
    }

    #[test]
    fn test_broadcast_empty_patch_id() {
        let mut daemon = RadicleSyncDaemon::new("rad:xyz", "abc123");
        daemon.add_seed_node("seed.radicle.dev:8776");
        assert!(daemon.broadcast_patch("").is_err());
    }

    #[test]
    fn test_reconcile_remote_heads() {
        let mut daemon = RadicleSyncDaemon::new("rad:xyz", "head-42");
        daemon.add_seed_node("seed1");
        daemon.add_seed_node("seed2");
        let count = daemon.reconcile_remote_heads().unwrap();
        assert_eq!(count, 2);
        assert_eq!(daemon.peer_heads.get("seed1").unwrap(), "head-42");
        assert_eq!(daemon.peer_heads.get("seed2").unwrap(), "head-42");
    }

    #[test]
    fn test_peer_timeout_handling() {
        let mut daemon = RadicleSyncDaemon::new("rad:xyz", "head-42");
        daemon.add_seed_node("seed-ok");
        daemon.add_seed_node("seed-timeout");
        let count = daemon.reconcile_remote_heads().unwrap();
        assert_eq!(count, 1);
        assert!(daemon.timed_out_peers.contains(&"seed-timeout".to_string()));
    }

    #[test]
    fn test_reconcile_no_seeds() {
        let mut daemon = RadicleSyncDaemon::new("rad:xyz", "head-42");
        assert!(daemon.reconcile_remote_heads().is_err());
    }
}
