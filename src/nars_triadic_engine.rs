#![allow(dead_code)]
// NARS truth-value calculator and Three-Value (Triadic) logic gate.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriadicState {
    CoherentPDPlus,
    DecoherentPDMinus,
    Indeterminate,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct NarsTruthValue {
    pub f: f64,
    pub c: f64,
}

impl NarsTruthValue {
    pub fn new(f: f64, c: f64) -> Self {
        Self { f, c }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NarsTriadicEngine {
    pub confidence_threshold: f64,
    pub high_frequency_threshold: f64,
    pub low_frequency_threshold: f64,
}

impl NarsTriadicEngine {
    pub fn new() -> Self {
        Self {
            confidence_threshold: 0.5,
            high_frequency_threshold: 0.7,
            low_frequency_threshold: 0.3,
        }
    }

    pub fn evaluate_triadic_gate(&self, truth: &NarsTruthValue) -> TriadicState {
        // AIKR: insufficient confidence -> indeterminate.
        if truth.c < self.confidence_threshold {
            return TriadicState::Indeterminate;
        }
        if truth.f >= self.high_frequency_threshold {
            TriadicState::CoherentPDPlus
        } else if truth.f <= self.low_frequency_threshold {
            TriadicState::DecoherentPDMinus
        } else {
            TriadicState::Indeterminate
        }
    }

    pub fn revise_truth(&self, old: NarsTruthValue, new_evidence: NarsTruthValue) -> NarsTruthValue {
        // Simple NARS revision: weighted average by confidence.
        let total_c = old.c + new_evidence.c;
        if total_c == 0.0 {
            return NarsTruthValue::new(0.5, 0.0);
        }
        let f = (old.f * old.c + new_evidence.f * new_evidence.c) / total_c;
        let c = total_c / (total_c + 1.0); // confidence grows with evidence
        NarsTruthValue::new(f, c.min(1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triadic_classification_coherent() {
        let engine = NarsTriadicEngine::new();
        let truth = NarsTruthValue::new(0.9, 0.8);
        assert_eq!(engine.evaluate_triadic_gate(&truth), TriadicState::CoherentPDPlus);
    }

    #[test]
    fn test_triadic_classification_decoherent() {
        let engine = NarsTriadicEngine::new();
        let truth = NarsTruthValue::new(0.1, 0.8);
        assert_eq!(engine.evaluate_triadic_gate(&truth), TriadicState::DecoherentPDMinus);
    }

    #[test]
    fn test_triadic_classification_indeterminate_low_confidence() {
        let engine = NarsTriadicEngine::new();
        let truth = NarsTruthValue::new(0.9, 0.2);
        assert_eq!(engine.evaluate_triadic_gate(&truth), TriadicState::Indeterminate);
    }

    #[test]
    fn test_triadic_classification_indeterminate_mid_frequency() {
        let engine = NarsTriadicEngine::new();
        let truth = NarsTruthValue::new(0.5, 0.8);
        assert_eq!(engine.evaluate_triadic_gate(&truth), TriadicState::Indeterminate);
    }

    #[test]
    fn test_truth_revision() {
        let engine = NarsTriadicEngine::new();
        let old = NarsTruthValue::new(0.2, 0.5);
        let new_ev = NarsTruthValue::new(0.8, 0.5);
        let revised = engine.revise_truth(old, new_ev);
        assert!((revised.f - 0.5).abs() < 1e-9);
        assert!(revised.c > 0.0 && revised.c <= 1.0);
    }

    #[test]
    fn test_zero_drift_invariant() {
        let engine = NarsTriadicEngine::new();
        // High confidence, high frequency -> coherent (zero drift).
        let truth = NarsTruthValue::new(0.95, 0.9);
        assert_eq!(engine.evaluate_triadic_gate(&truth), TriadicState::CoherentPDPlus);
    }
}
