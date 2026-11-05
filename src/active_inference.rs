#![allow(dead_code)]

use crate::geometry::RiemannianManifold;

pub struct ActiveInferenceAgent {
    pub agent_id: String,
    pub expected_states: Vec<f64>,
    pub precision: f64,
    pub manifold: RiemannianManifold,
}

impl ActiveInferenceAgent {
    pub fn new(agent_id: &str, expected_states: Vec<f64>, precision: f64, manifold: RiemannianManifold) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            expected_states,
            precision,
            manifold,
        }
    }

    pub fn calculate_free_energy(&self, sensory_input: &[f64]) -> Result<f64, String> {
        let geodesic_dist = self.manifold.geodesic_distance(&self.expected_states, sensory_input)?;
        let variational_free_energy = geodesic_dist * geodesic_dist * self.precision;
        Ok(variational_free_energy)
    }
}
