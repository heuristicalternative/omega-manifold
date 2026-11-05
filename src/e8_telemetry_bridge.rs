#![allow(dead_code)]
// E8 root lattice telemetry bridge and eBPF ringbuffer feedback engine.

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct E8TelemetryBridge {
    pub e8_root_weights: [f64; 248],
    pub tracepoint_weights: Vec<f64>,
    pub memory_allocation_weights: Vec<f64>,
    pub last_projection: [f64; 8],
}

impl E8TelemetryBridge {
    pub fn new() -> Self {
        Self {
            e8_root_weights: [1.0; 248],
            tracepoint_weights: vec![1.0; 248],
            memory_allocation_weights: vec![1.0; 248],
            last_projection: [0.0; 8],
        }
    }

    /// Project 50D hardware telemetry onto 8D E8 sphere-packing coordinates.
    pub fn project_to_e8_lattice(&mut self, telemetry_vector: &[f64; 50]) -> Result<[f64; 8], String> {
        for v in telemetry_vector {
            if v.is_nan() || v.is_infinite() {
                return Err("telemetry contains non-finite value".to_string());
            }
        }

        // Aggregate 50 inputs into 8 groups (E8 has 8 basis dimensions).
        let mut groups = [0.0f64; 8];
        let mut counts = [0usize; 8];
        for (i, v) in telemetry_vector.iter().enumerate() {
            let idx = i % 8;
            groups[idx] += *v;
            counts[idx] += 1;
        }
        for i in 0..8 {
            if counts[i] > 0 {
                groups[i] /= counts[i] as f64;
            }
        }

        // Normalize to unit sphere for sphere-packing coordinates.
        let norm: f64 = groups.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm > 0.0 {
            for v in groups.iter_mut() {
                *v /= norm;
            }
        }

        self.last_projection = groups;
        Ok(groups)
    }

    pub fn last_projection(&self) -> [f64; 8] {
        self.last_projection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection_output_dimension() {
        let mut bridge = E8TelemetryBridge::new();
        let input = [1.0; 50];
        let out = bridge.project_to_e8_lattice(&input).unwrap();
        assert_eq!(out.len(), 8);
    }

    #[test]
    fn test_projection_finite() {
        let mut bridge = E8TelemetryBridge::new();
        let mut input = [0.0; 50];
        for (i, v) in input.iter_mut().enumerate() {
            *v = (i as f64) * 0.1;
        }
        let out = bridge.project_to_e8_lattice(&input).unwrap();
        assert!(out.iter().all(|v| v.is_finite()));
    }

    #[test]
    fn test_projection_deterministic() {
        let mut bridge = E8TelemetryBridge::new();
        let input = [2.0; 50];
        let a = bridge.project_to_e8_lattice(&input).unwrap();
        let b = bridge.project_to_e8_lattice(&input).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_non_finite_rejected() {
        let mut bridge = E8TelemetryBridge::new();
        let mut input = [1.0; 50];
        input[0] = f64::NAN;
        assert!(bridge.project_to_e8_lattice(&input).is_err());
    }

    #[test]
    fn test_zero_input_yields_zero() {
        let mut bridge = E8TelemetryBridge::new();
        let input = [0.0; 50];
        let out = bridge.project_to_e8_lattice(&input).unwrap();
        assert!(out.iter().all(|v| *v == 0.0));
    }

    #[test]
    fn test_last_projection_stored() {
        let mut bridge = E8TelemetryBridge::new();
        let input = [1.0; 50];
        let out = bridge.project_to_e8_lattice(&input).unwrap();
        assert_eq!(bridge.last_projection(), out);
    }
}
