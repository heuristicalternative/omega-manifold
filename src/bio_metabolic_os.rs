#![allow(dead_code)]
// Bio-Metabolic OS resource and mutual credit clearing engine.

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BioMetabolicEngine {
    pub balances: HashMap<String, f64>,
    pub negentropy_rates: HashMap<String, f64>,
    pub routing_weights: HashMap<String, f64>,
}

impl BioMetabolicEngine {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            negentropy_rates: HashMap::new(),
            routing_weights: HashMap::new(),
        }
    }

    pub fn register_node(&mut self, node_id: &str, negentropy_rate: f64, routing_weight: f64) {
        self.balances.insert(node_id.to_string(), 0.0);
        self.negentropy_rates.insert(node_id.to_string(), negentropy_rate);
        self.routing_weights.insert(node_id.to_string(), routing_weight);
    }

    /// Route mutual credit based on local thermodynamic efficiency.
    /// Returns the transferred credit amount.
    pub fn route_metabolic_credits(
        &mut self,
        source_node: &str,
        target_node: &str,
        computational_load: f64,
    ) -> Result<f64, String> {
        if computational_load < 0.0 {
            return Err("computational load must be non-negative".to_string());
        }
        if !self.balances.contains_key(source_node) {
            return Err(format!("unknown source node: {}", source_node));
        }
        if !self.balances.contains_key(target_node) {
            return Err(format!("unknown target node: {}", target_node));
        }

        let efficiency = self.negentropy_rates.get(source_node).copied().unwrap_or(1.0);
        let weight = self.routing_weights.get(source_node).copied().unwrap_or(1.0);
        let amount = computational_load * efficiency * weight;

        // Thermodynamic bound: amount must be finite and non-negative.
        if !amount.is_finite() || amount < 0.0 {
            return Err("thermodynamic bound violated".to_string());
        }

        // Mutual credit transfer (zero-sum, no inflation).
        if let Some(src) = self.balances.get_mut(source_node) {
            *src -= amount;
        }
        if let Some(dst) = self.balances.get_mut(target_node) {
            *dst += amount;
        }

        Ok(amount)
    }

    pub fn total_supply(&self) -> f64 {
        self.balances.values().sum()
    }

    pub fn zero_drift(&self) -> bool {
        self.total_supply().abs() < 1e-9
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_conservation() {
        let mut engine = BioMetabolicEngine::new();
        engine.register_node("a", 1.0, 1.0);
        engine.register_node("b", 0.8, 1.0);
        let amount = engine.route_metabolic_credits("a", "b", 10.0).unwrap();
        assert!((amount - 10.0).abs() < 1e-9);
        assert!(engine.zero_drift());
        assert!((engine.balances["a"] + 10.0).abs() < 1e-9);
        assert!((engine.balances["b"] - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_thermodynamic_bounds() {
        let mut engine = BioMetabolicEngine::new();
        engine.register_node("a", 1.0, 1.0);
        engine.register_node("b", 1.0, 1.0);
        assert!(engine.route_metabolic_credits("a", "b", -1.0).is_err());
        assert!(engine.route_metabolic_credits("unknown", "b", 1.0).is_err());
        assert!(engine.route_metabolic_credits("a", "unknown", 1.0).is_err());
    }

    #[test]
    fn test_zero_drift_settlement() {
        let mut engine = BioMetabolicEngine::new();
        engine.register_node("x", 1.2, 0.9);
        engine.register_node("y", 0.7, 1.1);
        engine.route_metabolic_credits("x", "y", 5.0).unwrap();
        engine.route_metabolic_credits("y", "x", 2.0).unwrap();
        assert!(engine.zero_drift());
    }

    #[test]
    fn test_efficiency_scaling() {
        let mut engine = BioMetabolicEngine::new();
        engine.register_node("src", 2.0, 1.0);
        engine.register_node("dst", 1.0, 1.0);
        let amount = engine.route_metabolic_credits("src", "dst", 5.0).unwrap();
        assert!((amount - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_zero_load() {
        let mut engine = BioMetabolicEngine::new();
        engine.register_node("a", 1.0, 1.0);
        engine.register_node("b", 1.0, 1.0);
        let amount = engine.route_metabolic_credits("a", "b", 0.0).unwrap();
        assert!((amount).abs() < 1e-9);
        assert!(engine.zero_drift());
    }
}
