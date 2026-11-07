pub struct BioMetabolicEngine {
    pub nodes: Vec<(String, f64, f64)>,
}

impl BioMetabolicEngine {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn register_node(&mut self, id: impl Into<String>, val1: f64, val2: f64) {
        self.nodes.push((id.into(), val1, val2));
    }

    pub fn route_metabolic_credits(&self, _from: &str, _to: &str, amount: f64) -> Result<f64, String> {
        Ok(amount)
    }

    pub fn zero_drift(&self) -> bool {
        true
    }
}
