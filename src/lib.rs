pub mod health_monitor;
pub mod gossip_protocol;
pub mod conservation_checker;
pub mod ebpf_consumer;
pub mod mesh_sync;
pub mod telemetry_aggregator;
pub mod stigmergic_mesh;
pub mod active_inference;
pub mod geometry;
pub mod sheaf_cohomology;
pub mod negentropy;
pub mod gossip;
pub mod orchestrator;
pub mod ebpf_ringbuffer;
pub mod webgpu_compute;
pub mod agent_mcp_bridge;
pub mod telemetry_sync;
pub mod substrate_audit;
pub mod daemon_runner;
pub mod tensor_field_simulator;
pub mod omega_handshake;
pub mod self_heal;
pub mod manifold_routing;
pub mod state_sync;
pub mod zero_knowledge;
pub mod negentropy_sync;
pub mod vector_mesh;
pub mod gpu_compute;
pub mod vfs;
pub mod ebpf_probe;
pub mod mutual_credit;
pub mod wasm_runtime;
pub mod vault_archiver;
pub mod bio_metabolic_os;
pub mod profunctor_optic;
pub mod qdrant_adapter;
pub mod radicle_sync;
pub mod cluster_sim;

pub use vfs::Plan9Vfs;

pub mod plan9_vfs {
    pub use crate::vfs::Plan9Vfs;
}
