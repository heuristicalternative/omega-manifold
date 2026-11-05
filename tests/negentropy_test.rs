use omega_manifold::negentropy::NegentropySyncNode;

#[test]
fn test_negentropy_reconciliation() {
    let mut node_a = NegentropySyncNode::new("Node-A");
    let mut node_b = NegentropySyncNode::new("Node-B");

    node_a.insert_entry(100, "hash_v1".to_string());
    node_b.insert_entry(101, "hash_v2".to_string());

    let proof_a = node_a.generate_range_proof(100, 105);
    let reconciled_count = node_b.reconcile(&proof_a);

    assert_eq!(reconciled_count, 1);
    assert_eq!(node_b.generate_range_proof(100, 100), vec![(100, "hash_v1".to_string())]);
}
