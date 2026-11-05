use omega_manifold::orchestrator::OmegaOrchestrator;
use omega_manifold::gpu_compute::GpuComputePipeline;
use omega_manifold::telemetry_sync::TelemetryRingBuffer;

#[test]
fn test_omega_manifold_end_to_end() {
    // 1. Initialize GPU Compute Pipeline
    let gpu = GpuComputePipeline::new("Melusine-GPU-Node", 64);
    let shader = "@compute @workgroup_size(64) fn main() {}";
    let compiled_shader = gpu.compile_wgsl_shader(shader);
    assert!(compiled_shader.is_ok());

    // 2. Initialize Telemetry Ring Buffer & eBPF trace hooks
    let mut ring = TelemetryRingBuffer::new(32);
    ring.push_event(2000, "e2e_kernel_probe_hash".to_string());

    // 3. Initialize Unified Orchestrator & Run Cycle
    let mut orch = OmegaOrchestrator::new(
        "Melusine-E2E-Master",
        vec![2.0, 4.0],
        1.0,
        0.5,
    );

    ring.sync_to_negentropy(&mut orch.sync_node);
    let fe_result = orch.step(&vec![2.1, 3.9]);

    assert!(fe_result.is_ok());
    assert!((fe_result.unwrap() - 0.02).abs() < 1e-5);
    assert_eq!(orch.gossip.active_peer_count(), 0);
}
