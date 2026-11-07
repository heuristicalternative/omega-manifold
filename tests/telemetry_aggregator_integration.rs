use omega_manifold::telemetry_aggregator::TelemetryAggregator;

#[test]
fn test_telemetry_aggregator_pipeline() {
    let aggregator = TelemetryAggregator::new(10);
    aggregator.push_event(1005, "ebpf_trace_hash_beta".to_string(), 0.95);
    
    let snapshot = aggregator.snapshot_events();
    assert_eq!(snapshot.len(), 1);
    assert_eq!(snapshot[0].timestamp, 1005);
    assert_eq!(snapshot[0].trace_hash, "ebpf_trace_hash_beta");
}
