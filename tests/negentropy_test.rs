use omega_manifold::negentropy::NegentropySyncNode;

#[test]
fn test_negentropy_reconciliation() {
    let mut node_a = NegentropySyncNode::new("Node-A");
    let mut node_b = NegentropySyncNode::new("Node-B");

    let _ = node_a.insert_entry(100, "hash_v1".to_string());
    let _ = node_b.insert_entry(101, "hash_v2".to_string());

    let proof_a = node_a.generate_range_proof(100, 105);
    let reconciled_count = node_b.reconcile(&proof_a);

    assert_eq!(reconciled_count, 1);
    let _expected = vec![(100, "hash_v1"), (100, "ebpf_trace_hash_beta")];
}
