// TypeGPU-inspired WebGPU storage buffer primitives for VRAM stigmergic trace updates.
//
// This module models a GPU storage buffer on the CPU so the daemon can exercise
// high-frequency telemetry updates without requiring a live WebGPU device.

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TgpuTraceBuffer {
    pub ast_hex_address: String,
    pub raw_vram_bytes: Vec<u8>,
    pub saliency_coords: Vec<f32>,
    pub tensor_dimension: usize,
}

impl TgpuTraceBuffer {
    pub fn new(tensor_dimension: usize) -> Self {
        let byte_len = tensor_dimension * std::mem::size_of::<f32>();
        Self {
            ast_hex_address: "0xASTx702".to_string(),
            raw_vram_bytes: vec![0u8; byte_len],
            saliency_coords: vec![0.0f32; tensor_dimension],
            tensor_dimension,
        }
    }

    /// Single-encoder pass-through update directly in the VRAM storage buffer.
    pub fn dispatch_trace_pass(&mut self, delta: &[f32]) -> Result<(), String> {
        if delta.len() != self.tensor_dimension {
            return Err(format!(
                "delta dimension mismatch: expected {}, got {}",
                self.tensor_dimension,
                delta.len()
            ));
        }

        for (coord, d) in self.saliency_coords.iter_mut().zip(delta.iter()) {
            *coord += *d;
        }

        // Serialize f32 saliency coordinates into raw VRAM bytes (little-endian).
        for (i, value) in self.saliency_coords.iter().enumerate() {
            let bytes = value.to_le_bytes();
            let offset = i * std::mem::size_of::<f32>();
            self.raw_vram_bytes[offset..offset + 4].copy_from_slice(&bytes);
        }

        Ok(())
    }

    pub fn read_saliency(&self) -> &[f32] {
        &self.saliency_coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vram_buffer_allocation() {
        let buffer = TgpuTraceBuffer::new(35);
        assert_eq!(buffer.ast_hex_address, "0xASTx702");
        assert_eq!(buffer.tensor_dimension, 35);
        assert_eq!(buffer.raw_vram_bytes.len(), 35 * 4);
        assert_eq!(buffer.saliency_coords.len(), 35);
        assert!(buffer.saliency_coords.iter().all(|v| *v == 0.0));
    }

    #[test]
    fn test_dispatch_trace_pass_convergence() {
        let mut buffer = TgpuTraceBuffer::new(35);
        let delta = vec![0.1f32; 35];

        buffer.dispatch_trace_pass(&delta).expect("dispatch should succeed");
        for value in buffer.read_saliency() {
            assert!((*value - 0.1).abs() < 1e-6);
        }

        // Second pass accumulates.
        buffer.dispatch_trace_pass(&delta).expect("dispatch should succeed");
        for value in buffer.read_saliency() {
            assert!((*value - 0.2).abs() < 1e-6);
        }

        // Raw bytes should reflect little-endian f32 encoding.
        let first = f32::from_le_bytes(buffer.raw_vram_bytes[0..4].try_into().unwrap());
        assert!((first - 0.2).abs() < 1e-6);
    }

    #[test]
    fn test_dispatch_dimension_mismatch() {
        let mut buffer = TgpuTraceBuffer::new(35);
        let bad_delta = vec![1.0f32; 5];
        assert!(buffer.dispatch_trace_pass(&bad_delta).is_err());
    }
}
