use omega_manifold::health_monitor::ClusterHealthMonitor;

#[test]
fn test_cluster_health_monitor_operations() {
    let mut monitor = ClusterHealthMonitor::new("melusine-master-node");
    assert!(monitor.is_healthy());
    
    let hb1 = monitor.pulse();
    assert_eq!(hb1, 1);

    monitor.set_health_status(false);
    assert!(!monitor.is_healthy());
}
