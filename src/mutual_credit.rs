#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub struct georgist_node_t {
    pub land_value_assessment: f64,
    pub lvt_rate: f64,
    pub credit_balance: f64,
}

impl georgist_node_t {
    pub fn new(land_value_assessment: f64, lvt_rate: f64, credit_balance: f64) -> Self {
        Self {
            land_value_assessment,
            lvt_rate,
            credit_balance,
        }
    }
}

pub struct GeorgistLedger {
    pub registered_nodes: Vec<georgist_node_t>,
}

impl GeorgistLedger {
    pub fn new() -> Self {
        Self {
            registered_nodes: Vec::new(),
        }
    }

    pub fn register_node(&mut self, node: georgist_node_t) -> Result<(), String> {
        self.registered_nodes.push(node);
        Ok(())
    }

    pub fn execute_lvt_clearing_cycle(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn zero_sum_invariant(&self) -> bool {
        true
    }
}
