use omega_manifold::telemetry_store::TelemetryStore;
use std::fs;

#[test]
fn test_telemetry_store_append() {
    let log_file = "test_telemetry.log";
    let store = TelemetryStore::new(log_file);
    
    let res = store.append_snapshot(3072, "ebpf_kernel_trace_alpha", 1.05);
    assert!(res.is_ok());

    let content = fs::read_to_string(log_file).unwrap();
    assert!(content.contains("3072,ebpf_kernel_trace_alpha,1.05"));

    let _ = fs::remove_file(log_file);
}
