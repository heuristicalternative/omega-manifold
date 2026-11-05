use omega_manifold::negentropy::NegentropySyncNode;
use omega_manifold::telemetry_sync::TelemetryRingBuffer;

#[test]
fn test_telemetry_ring_buffer_sync() {
    let mut ring = TelemetryRingBuffer::new(5);
    ring.push_event(1000, "ebpf_trace_hash_alpha".to_string());
    ring.push_event(1001, "ebpf_trace_hash_beta".to_string());

    let mut negentropy_node = NegentropySyncNode::new("Melusine-Node");
    ring.sync_to_negentropy(&mut negentropy_node);

    let proof = negentropy_node.generate_range_proof(1000, 1005);
    assert_eq!(proof.len(), 2);
    assert_eq!(proof[0], (1000, "ebpf_trace_hash_alpha".to_string()));
    assert_eq!(proof[1], (1001, "ebpf_trace_hash_beta".to_string()));
}
