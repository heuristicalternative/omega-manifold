use omega_manifold::webgpu_compute::WebGpuComputePipeline;

#[test]
fn test_webgpu_compute_pipeline_execution() {
    let pipeline = WebGpuComputePipeline::new("Melusine-Node", 35);
    let mut tensor = vec![1.0f32; 35];
    
    let result = pipeline.execute_compute_pipeline(&mut tensor);
    assert!(result.is_ok());
    assert_eq!(tensor.len(), 35);
}
