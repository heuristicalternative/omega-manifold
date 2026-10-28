use std::process::Command;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // 1. Probe Target Architecture and OS
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    println!("cargo:warning=[OMEGA_BUILD] Target Architecture: {} | OS: {}", target_arch, target_os);

    // 2. Check for CUDA availability
    let nvcc_check = Command::new("nvcc").arg("--version").status();
    if nvcc_check.is_ok() && nvcc_check.unwrap().success() {
        println!("cargo:rustc-cfg=feature=\"cuda\"");
        println!("cargo:warning=[OMEGA_BUILD] CUDA accelerator detected. Enabling GPU pipeline.");
    }

    // 3. Check for eBPF kernel support (Linux headers)
    if target_os == "linux" {
        let bpf_check = std::path::Path::new("/sys/fs/bpf").exists();
        if bpf_check {
            println!("cargo:rustc-cfg=feature=\"ebpf\"");
            println!("cargo:warning=[OMEGA_BUILD] eBPF subsystem available.");
        }
    }
}
