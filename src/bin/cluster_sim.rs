use omega_manifold::telemetry_aggregator::TelemetryAggregator;
use omega_manifold::cluster_sim::ClusterSimCoordinator;

fn main() {
    let aggregator = TelemetryAggregator::new(100);
    aggregator.push_event(1005, "ebpf_cluster_trace_init".to_string(), 1.0);
    let coordinator = ClusterSimCoordinator::new("omega-cluster-alpha");
    println!("[CLUSTER_SIM] Initialized cluster coordinator for: {}", coordinator.cluster_id);
}
