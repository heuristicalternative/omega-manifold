use omega_manifold::webgpu_compute::WebGpuComputePipeline;

#[test]
fn test_webgpu_compute_pipeline_generation() {
    let pipeline = WebGpuComputePipeline::new("Melusine-Node", 35);
    assert!(pipeline.verify_pipeline());
    let shader = pipeline.generate_compute_shader_source();
    assert!(shader.contains("@compute @workgroup_size(64)"));
    assert!(shader.contains("tensor_data"));
}
