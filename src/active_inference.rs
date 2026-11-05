#![allow(dead_code)]

pub struct ActiveInferenceAgent {
    pub agent_id: String,
    pub expected_states: Vec<f64>,
    pub precision: f64,
}

impl ActiveInferenceAgent {
    pub fn new(agent_id: &str, expected_states: Vec<f64>, precision: f64) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            expected_states,
            precision,
        }
    }

    pub fn calculate_free_energy(&self, sensory_input: &[f64]) -> f64 {
        let mut variational_free_energy = 0.0;
        for (exp, obs) in self.expected_states.iter().zip(sensory_input.iter()) {
            let diff = exp - obs;
            variational_free_energy += diff * diff * self.precision;
        }
        variational_free_energy
    }
}
