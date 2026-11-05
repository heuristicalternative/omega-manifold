#![allow(dead_code)]
// Tensor-level triadic logic gate and NARS metric tensor modulator.

use crate::nars_triadic_engine::{NarsTriadicEngine, NarsTruthValue, TriadicState};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TensorTriadicGate {
    pub truth: NarsTruthValue,
    pub state: TriadicState,
    pub modulation_matrix: [f64; 9],
    pub nars_engine: NarsTriadicEngine,
}

impl TensorTriadicGate {
    pub fn new() -> Self {
        Self {
            truth: NarsTruthValue::new(0.5, 0.5),
            state: TriadicState::Indeterminate,
            modulation_matrix: [1.0; 9],
            nars_engine: NarsTriadicEngine::new(),
        }
    }

    pub fn evaluate_and_gate_tensor(
        &mut self,
        input_tensor: &[f64; 9],
        truth: &NarsTruthValue,
    ) -> Result<[f64; 9], String> {
        for v in input_tensor {
            if v.is_nan() || v.is_infinite() {
                return Err("input tensor contains non-finite value".to_string());
            }
        }

        self.truth = *truth;
        self.state = self.nars_engine.evaluate_triadic_gate(truth);

        let factor = match self.state {
            TriadicState::CoherentPDPlus => 1.1,      // acceleration
            TriadicState::DecoherentPDMinus => 0.5,  // damping
            TriadicState::Indeterminate => 1.0,      // sandbox isolation (pass-through)
        };

        let mut output = [0.0f64; 9];
        for i in 0..9 {
            output[i] = input_tensor[i] * factor * self.modulation_matrix[i];
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nars_triadic_engine::NarsTruthValue;

    #[test]
    fn test_coherent_acceleration() {
        let mut gate = TensorTriadicGate::new();
        let input = [1.0; 9];
        let truth = NarsTruthValue::new(0.9, 0.8);
        let out = gate.evaluate_and_gate_tensor(&input, &truth).unwrap();
        assert_eq!(gate.state, TriadicState::CoherentPDPlus);
        for v in out {
            assert!((v - 1.1).abs() < 1e-9);
        }
    }

    #[test]
    fn test_decoherent_damping() {
        let mut gate = TensorTriadicGate::new();
        let input = [1.0; 9];
        let truth = NarsTruthValue::new(0.1, 0.8);
        let out = gate.evaluate_and_gate_tensor(&input, &truth).unwrap();
        assert_eq!(gate.state, TriadicState::DecoherentPDMinus);
        for v in out {
            assert!((v - 0.5).abs() < 1e-9);
        }
    }

    #[test]
    fn test_indeterminate_isolation() {
        let mut gate = TensorTriadicGate::new();
        let input = [2.0; 9];
        let truth = NarsTruthValue::new(0.9, 0.2); // low confidence
        let out = gate.evaluate_and_gate_tensor(&input, &truth).unwrap();
        assert_eq!(gate.state, TriadicState::Indeterminate);
        for v in out {
            assert!((v - 2.0).abs() < 1e-9);
        }
    }

    #[test]
    fn test_non_finite_rejected() {
        let mut gate = TensorTriadicGate::new();
        let mut input = [1.0; 9];
        input[0] = f64::NAN;
        let truth = NarsTruthValue::new(0.9, 0.8);
        assert!(gate.evaluate_and_gate_tensor(&input, &truth).is_err());
    }

    #[test]
    fn test_modulation_matrix_applied() {
        let mut gate = TensorTriadicGate::new();
        gate.modulation_matrix = [2.0; 9];
        let input = [1.0; 9];
        let truth = NarsTruthValue::new(0.9, 0.8);
        let out = gate.evaluate_and_gate_tensor(&input, &truth).unwrap();
        for v in out {
            assert!((v - 2.2).abs() < 1e-9);
        }
    }
}
