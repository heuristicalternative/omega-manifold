#![allow(dead_code)]

pub struct WebGpuComputePipeline {
    pub device_id: String,
    pub capacity: usize,
}

impl WebGpuComputePipeline {
    pub fn new(device_id: impl Into<String>, capacity: usize) -> Self {
        Self {
            device_id: device_id.into(),
            capacity,
        }
    }

    pub fn compile_wgsl_shader(&self, _shader_source: &str) -> Result<bool, String> {
        Ok(true)
    }
}
