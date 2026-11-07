use omega_manifold::gpu_compute::GpuComputePipeline;

#[test]
fn test_gpu_compute_pipeline() {
    let pipeline = GpuComputePipeline::new("Melusine-AMD-GPU", 256);
    let shader = "@compute @workgroup_size(256) fn main() {}";
    let result = pipeline.compile_wgsl_shader(shader);
    assert!(result.is_ok());
    assert!(result.is_ok());
}
