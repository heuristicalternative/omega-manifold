use omega_manifold::cluster_sim::ClusterSimCoordinator;

fn main() {
    let coordinator = ClusterSimCoordinator::new("omega-cluster-alpha");
    println!("[CLUSTER_SIM] Initialized cluster coordinator for: {}", coordinator.cluster_id);
}
