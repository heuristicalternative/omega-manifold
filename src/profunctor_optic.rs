#![allow(dead_code)]

// Bidirectional coend profunctor optics for invariant boundary transformations.

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProfunctorOptic {
    pub boundary_id: String,
    pub forward_gain: f64,
    pub backward_gain: f64,
}

impl ProfunctorOptic {
    pub fn new(boundary_id: &str) -> Self {
        Self {
            boundary_id: boundary_id.to_string(),
            forward_gain: 1.0,
            backward_gain: 0.5,
        }
    }

    /// Forward projection: internal state -> environment offer.
    pub fn project_forward(&self, internal_state: f64) -> f64 {
        internal_state * self.forward_gain
    }

    /// Backward feedback: environmental stress/demurrage -> updated internal capacity.
    pub fn feedback_backward(&self, stress: f64, capacity: f64) -> f64 {
        capacity - stress * self.backward_gain
    }

    /// Coend quotienting: external interactions are normalized to the invariant
    /// boundary interface, preventing direct access to internal state.
    pub fn coend_quotient(&self, external: f64) -> f64 {
        external.rem_euclid(1.0)
    }

    /// Compose two profunctor optics (forward then backward) across a boundary.
    pub fn compose(&self, other: &Self, internal_state: f64, stress: f64) -> (f64, f64) {
        let offer = other.project_forward(self.project_forward(internal_state));
        let capacity = self.feedback_backward(stress, other.feedback_backward(stress, 1.0));
        (offer, capacity)
    }

    /// Transform a tensor payload through the forward optic and coend quotient.
    pub fn transform_tensor(&self, values: &[f64]) -> Vec<f64> {
        values
            .iter()
            .map(|v| self.coend_quotient(self.project_forward(*v)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_projection() {
        let optic = ProfunctorOptic::new("boundary_0");
        assert!((optic.project_forward(2.0) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_backward_feedback() {
        let optic = ProfunctorOptic::new("boundary_0");
        let updated = optic.feedback_backward(1.0, 10.0);
        assert!((updated - 9.5).abs() < 1e-9);
    }

    #[test]
    fn test_coend_quotient_boundary_isolation() {
        let optic = ProfunctorOptic::new("boundary_0");
        assert!((optic.coend_quotient(3.7) - 0.7).abs() < 1e-9);
        assert!((optic.coend_quotient(-0.3) - 0.7).abs() < 1e-9);
    }

    #[test]
    fn test_compose_optics() {
        let a = ProfunctorOptic::new("b1");
        let b = ProfunctorOptic::new("b2");
        let (offer, capacity) = a.compose(&b, 2.0, 0.5);
        assert!(offer.is_finite());
        assert!(capacity.is_finite());
    }

    #[test]
    fn test_transform_tensor() {
        let optic = ProfunctorOptic::new("b");
        let out = optic.transform_tensor(&[0.2, 1.3, 2.7]);
        assert_eq!(out.len(), 3);
        for v in out {
            assert!(v >= 0.0 && v < 1.0);
        }
    }
}
