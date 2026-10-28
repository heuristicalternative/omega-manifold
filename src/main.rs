fn main() {
    println!("[OMEGA_DAEMON] Initializing Omega Manifold node daemon...");
    println!("[OMEGA_DAEMON] Baseline: Omega-G Baseline v.16.78");

    let mut runner = omega_manifold::daemon_runner::OmegaDaemonRunner::new();
    if let Err(e) = runner.run_full_lifecycle(3) {
        eprintln!("[DAEMON_RUNNER_ERR] {}", e);
    } else {
        println!("[OMEGA_DAEMON] Node operational, cluster-interlocked, and synchronized.");
    }
}
