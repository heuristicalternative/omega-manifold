// Unified daemon lifecycle runner for the Omega Manifold.

use crate::cybernetic_loop::CyberneticCoordinator;
use crate::ostrom_governance::CommonPoolResource;
use crate::qdrant_adapter::{AstHexCoordinate, OmegaQdrantAdapter, OmegaStatePayload};
use crate::radicle_sync::RadicleSyncDaemon;
use crate::shm_bridge::ShmBridge;
use crate::state_sync::ManifoldSnapshot;
use crate::vault_archiver::VaultArchiver;

#[allow(dead_code)]
pub struct OmegaDaemonRunner {
    pub baseline: String,
    pub ast_hex: String,
    pub checkpoint_path: String,
    pub snapshot_path: String,
}

impl OmegaDaemonRunner {
    pub fn new() -> Self {
        Self {
            baseline: "Omega-G Baseline v.16.78".to_string(),
            ast_hex: "0xASTx702".to_string(),
            checkpoint_path: "/tmp/opencode/omega_daemon.checkpoint".to_string(),
            snapshot_path: "/tmp/opencode/omega_daemon.snapshot".to_string(),
        }
    }

    pub fn run_full_lifecycle(&mut self, cycles: usize) -> Result<(), String> {
        // Startup: eBPF probe.
        let ebpf = crate::ebpf_probe::EbpfProbe::new();
        if ebpf.is_available() {
            let _ = ebpf.attach_shm_tracepoint();
            let _ = ebpf.attach_socket_tracepoint();
        }

        // VFS mount.
        let mut vfs = crate::plan9_vfs::Plan9Vfs::new("/sys/omega");
        let _ = vfs.write_file("/sys/omega/manifold_state", b"epoch=107");

        // Qdrant vector index.
        let mut qdrant = OmegaQdrantAdapter::new("omega_states");

        // ShmBridge checkpoint substrate.
        let mut shm = ShmBridge::new(64);

        // Radicle mesh sync.
        let mut radicle = RadicleSyncDaemon::new("rad:omega-daemon", "head-omega");
        radicle.add_seed_node("seed.radicle.dev:8776");

        // Cybernetic coordinator.
        let mut cybernetic = CyberneticCoordinator::new();
        cybernetic.ingest_hardware_metrics(&[1.0, 1.0, 1.0, 1.0]);

        // Ostrom governance.
        let mut cpr = CommonPoolResource::new("manifold_capacity", 1000.0, 200.0);
        cpr.set_quota("melusine_node_01", 100.0);

        for cycle in 0..cycles {
            // Continuous cybernetic loop.
            cybernetic.run_cybernetic_cycle()?;

            // Periodic vector indexing.
            let payload = OmegaStatePayload {
                point_id: format!("cycle-{}", cycle),
                dimension: 35,
                baseline_status: "G_Universal_Baseline".to_string(),
                ast_hex: AstHexCoordinate {
                    x: 0,
                    y: 0,
                    z: 0,
                    node_id: "melusine_node_01".to_string(),
                    symbol_path: self.ast_hex.clone(),
                },
                logic_set_tags: vec!["omega".to_string(), "cycle".to_string()],
                timestamp: cycle as u64,
            };
            let _ = qdrant.store_state_point(payload);

            // VRAM persistence check.
            shm.segments.clear();
            shm.add_segment(vec![cycle as u8; 4])?;
            shm.flush_vram_traces(&self.checkpoint_path)?;

            // Radicle reconciliation (best effort).
            let _ = radicle.reconcile_remote_heads();
        }

        // GDrive sync archival reconciliation gate.
        let gdrive_manifest = crate::gdrive_sync::GDriveSyncManifest::new();
        let _ = gdrive_manifest.evaluate_sheaf_cohomology("0xASTx702_OmegaState");

        // Graceful shutdown: persist snapshot.
        let snapshot = ManifoldSnapshot::new(
            107,
            self.ast_hex.clone(),
            vec![0.0; 35],
            vec![("melusine_node_01".to_string(), 0.0)],
        );
        snapshot.export_snapshot(&self.snapshot_path).map_err(|e| e.to_string())?;

        // Persist complete vault archive.
        let mut archiver = VaultArchiver::new();
        archiver.metric_tensors = vec![0.0; 35];
        archiver.ast_coords = vec![(0, 0, 0)];
        archiver.fingerprints = vec!["omega-vault-v16.78".to_string()];
        let vault_path = "/home/dante/Documents/OmegaManifold_Local/vault/omega_state_archive_v16.78.json";
        archiver.export_vault_archive(vault_path).map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_lifecycle_startup() {
        let mut runner = OmegaDaemonRunner::new();
        assert!(runner.run_full_lifecycle(1).is_ok());
    }

    #[test]
    fn test_daemon_multi_cycle() {
        let mut runner = OmegaDaemonRunner::new();
        assert!(runner.run_full_lifecycle(3).is_ok());
    }

    #[test]
    fn test_daemon_graceful_shutdown_persists_snapshot() {
        let mut runner = OmegaDaemonRunner::new();
        runner.snapshot_path = "/tmp/opencode/daemon_runner_snapshot.txt".to_string();
        assert!(runner.run_full_lifecycle(1).is_ok());
        // Snapshot file should exist.
        assert!(std::path::Path::new(&runner.snapshot_path).exists());
        let _ = std::fs::remove_file(&runner.snapshot_path);
        let _ = std::fs::remove_file("/tmp/opencode/omega_daemon.checkpoint");
    }
}
