// AST Hex Coordinate: 0xOM16-0x4521-0xRUST-0x9F41-0xHOYER_TRIAD_NOVATION
// Grounding Protocol: Hans-Florian Hoyer 3-Party Triad Novation & Cyclic Obligation Collapse

use std::collections::HashMap;

pub struct TriadNovationEngine {
    pub h3_grid_id: String,
    pub obligations: HashMap<(String, String), f64>,
}

impl TriadNovationEngine {
    pub fn new(h3_grid_id: &str) -> Self {
        Self {
            h3_grid_id: h3_grid_id.to_string(),
            obligations: HashMap::new(),
        }
    }

    pub fn add_obligation(&mut self, debtor: &str, creditor: &str, amount: f64) {
        let key = (debtor.to_string(), creditor.to_string());
        *self.obligations.entry(key).or_insert(0.0) += amount;
    }

    pub fn execute_triad_collapse(&mut self, node_a: &str, node_b: &str, node_c: &str) -> Option<f64> {
        let debt_ab = *self.obligations.get(&(node_a.to_string(), node_b.to_string()))?;
        let debt_bc = *self.obligations.get(&(node_b.to_string(), node_c.to_string()))?;
        let debt_ca = *self.obligations.get(&(node_c.to_string(), node_a.to_string()))?;

        let min_clearing = debt_ab.min(debt_bc).min(debt_ca);
        if min_clearing > 0.0 {
            self.add_obligation(node_a, node_b, -min_clearing);
            self.add_obligation(node_b, node_c, -min_clearing);
            self.add_obligation(node_c, node_a, -min_clearing);
            println!("[HoyerTriadEngine] Collapsed 3-party cycle {{{}, {}, {}}} | Cleared Amount: {:.2}", 
                     node_a, node_b, node_c, min_clearing);
            return Some(min_clearing);
        }
        None
    }
}

fn main() {
    let mut engine = TriadNovationEngine::new("0x892a1072b57ffff");
    engine.add_obligation("did:nostr:alpha", "did:nostr:beta", 1500.0);
    engine.add_obligation("did:nostr:beta", "did:nostr:gamma", 1200.0);
    engine.add_obligation("did:nostr:gamma", "did:nostr:alpha", 1800.0);

    println!("--- EXECUTING HOYER TRIAD NOVATION IN RUST ---");
    engine.execute_triad_collapse("did:nostr:alpha", "did:nostr:beta", "did:nostr:gamma");
}
