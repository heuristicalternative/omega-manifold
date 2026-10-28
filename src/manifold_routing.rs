#![allow(dead_code)]

// Stigmergic event bus and dynamic manifold routing engine for Omega Manifold.

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteDecision {
    P2PGossipMesh,
    WasmRuntimeGate,
    GeorgistCreditLedger,
    Drop,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ManifoldSignal {
    pub source_coords: Vec<f64>,
    pub target_ast_hex: String,
    pub tensor_payload: Vec<f64>,
}

impl ManifoldSignal {
    pub fn new(source_coords: Vec<f64>, target_ast_hex: String, tensor_payload: Vec<f64>) -> Self {
        Self {
            source_coords,
            target_ast_hex,
            tensor_payload,
        }
    }

    pub fn is_well_formed(&self) -> bool {
        self.target_ast_hex == "0xASTx702"
            && !self.tensor_payload.is_empty()
            && self.source_coords.len() == self.tensor_payload.len()
    }
}

use crate::profunctor_optic::ProfunctorOptic;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct StigmergicRouter {
    pub queue: Vec<ManifoldSignal>,
    pub dispatched: Vec<RouteDecision>,
}

impl StigmergicRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&mut self, signal: ManifoldSignal) {
        self.queue.push(signal);
    }

    /// Tensor curvature: L2 norm of the 35D payload.
    pub fn evaluate_tensor_curvature(&self, signal: &ManifoldSignal) -> f64 {
        signal
            .tensor_payload
            .iter()
            .map(|v| v * v)
            .sum::<f64>()
            .sqrt()
    }

    pub fn route(&self, signal: &ManifoldSignal) -> RouteDecision {
        if !signal.is_well_formed() {
            return RouteDecision::Drop;
        }

        let curvature = self.evaluate_tensor_curvature(signal);
        if curvature < 0.5 {
            RouteDecision::P2PGossipMesh
        } else if curvature < 1.5 {
            RouteDecision::WasmRuntimeGate
        } else {
            RouteDecision::GeorgistCreditLedger
        }
    }

    pub fn transform_signal(&self, signal: &mut ManifoldSignal) {
        let optic = ProfunctorOptic::new("router_boundary");
        signal.tensor_payload = optic.transform_tensor(&signal.tensor_payload);
    }

    pub fn dispatch_next(&mut self) -> Option<RouteDecision> {
        let mut signal = self.queue.pop()?;
        self.transform_signal(&mut signal);
        let decision = self.route(&signal);
        self.dispatched.push(decision.clone());
        Some(decision)
    }

    pub fn dispatch_all(&mut self) -> Vec<RouteDecision> {
        let mut decisions = Vec::new();
        while let Some(decision) = self.dispatch_next() {
            decisions.push(decision);
        }
        decisions
    }
}

pub fn apply_memento_compression(signal: &ManifoldSignal) -> Result<Vec<u8>, String> {
    let cache = crate::memento_cache::MementoCache::new(4);
    let input: Vec<f32> = signal.tensor_payload.iter().map(|v| *v as f32).collect();
    cache.compress_context(&input, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signal_with_payload(payload: f64) -> ManifoldSignal {
        ManifoldSignal::new(
            vec![0.0; 35],
            "0xASTx702".to_string(),
            vec![payload; 35],
        )
    }

    #[test]
    fn test_low_curvature_routes_to_p2p() {
        let router = StigmergicRouter::new();
        let signal = signal_with_payload(0.01); // curvature ~0.059
        assert_eq!(router.route(&signal), RouteDecision::P2PGossipMesh);
    }

    #[test]
    fn test_mid_curvature_routes_to_wasm() {
        let router = StigmergicRouter::new();
        let signal = signal_with_payload(0.1); // curvature ~0.592
        assert_eq!(router.route(&signal), RouteDecision::WasmRuntimeGate);
    }

    #[test]
    fn test_high_curvature_routes_to_ledger() {
        let router = StigmergicRouter::new();
        let signal = signal_with_payload(1.0); // curvature ~5.916
        assert_eq!(router.route(&signal), RouteDecision::GeorgistCreditLedger);
    }

    #[test]
    fn test_malformed_signal_drops() {
        let router = StigmergicRouter::new();
        let signal = ManifoldSignal::new(vec![0.0; 35], "bad-address".to_string(), vec![0.1; 35]);
        assert_eq!(router.route(&signal), RouteDecision::Drop);
    }

    #[test]
    fn test_dispatch_all() {
        let mut router = StigmergicRouter::new();
        router.enqueue(signal_with_payload(0.01));
        router.enqueue(signal_with_payload(0.1));
        router.enqueue(signal_with_payload(1.0));

        let decisions = router.dispatch_all();
        assert_eq!(decisions.len(), 3);
        assert_eq!(router.queue.len(), 0);
        assert_eq!(router.dispatched.len(), 3);
    }
}
