use std::sync::Arc;
use omega_manifold::telemetry_aggregator::TelemetryAggregator;
use omega_manifold::ebpf_consumer::EbpfConsumerLoop;

#[test]
fn test_ebpf_consumer_polling_pipeline() {
    let aggregator = Arc::new(TelemetryAggregator::new(100));
    let mut consumer = EbpfConsumerLoop::new(aggregator.clone());

    assert!(!consumer.is_active());
    let res = consumer.poll_ring_buffer();
    assert!(res.is_ok());
    assert!(consumer.is_active());

    let snapshot = aggregator.snapshot_events();
    assert_eq!(snapshot.len(), 1);
    assert_eq!(snapshot[0].trace_hash, "ebpf_kernel_trace_alpha");
    assert_eq!(snapshot[0].timestamp, 3072);
}
