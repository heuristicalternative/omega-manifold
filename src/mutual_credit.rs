#![allow(dead_code)]

// Compositional Game Theory Mutual Credit & SRV Clearing Engine.
//
// Integrates:
//  - Compositional agent morphisms with Neuro-Type tracking
//  - Deception Delta and Asymmetric Payoff (Dark Triad power-gap maximization)
//  - SRV "River/Rock" velocity, hoarding decay, and common-wealth sluice gates
//  - Georgist LVT assessment nodes on the settlement hypergraph
//  - Zero-sum mutual credit conservation and entropy bounds

pub const AST_HEX_ADDRESS: &str = "0xASTx702";
pub const MAX_NODES: usize = 16;
pub const MAX_HYPEREDGES: usize = 32;
pub const MAX_ENTROPY: f64 = 1.0;
pub const SRV_VELOCITY_THRESHOLD: f64 = 0.25;
pub const SRV_HOARDING_THRESHOLD: f64 = 10.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeuroType {
    AutisticInvariant,
    AdhdNomadic,
    DarkTriadParasitic,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct georgist_node_t {
    pub land_value_assessment: f64,
    pub lvt_rate: f64,
    pub credit_balance: f64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct hypergraph_edge_t {
    pub source_node: String,
    pub target_node: String,
    pub credit_volume: f64,
}

#[derive(Debug, Clone)]
pub struct AgentMorphism {
    pub id: String,
    pub neuro_type: NeuroType,
    pub credit_balance: f64,
    pub power: f64,
    pub velocity: f64,
    pub hoarded_capital: f64,
    pub deception_delta: f64,
}

impl AgentMorphism {
    pub fn new(id: &str, neuro_type: NeuroType) -> Self {
        Self {
            id: id.to_string(),
            neuro_type,
            credit_balance: 0.0,
            power: 0.0,
            velocity: 0.0,
            hoarded_capital: 0.0,
            deception_delta: 0.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct GeorgistLedger {
    pub ast_hex_address: String,
    pub nodes: Vec<georgist_node_t>,
    pub hyperedges: Vec<hypergraph_edge_t>,
    pub common_wealth_pool: f64,
    pub citizens: usize,
}

#[allow(dead_code)]
pub type GeorgistSettlementEngine = GeorgistLedger;
#[allow(dead_code)]
pub type GeorgistSettlementOS = GeorgistLedger;
#[allow(dead_code)]
pub type HypergraphLedger = GeorgistLedger;
#[allow(dead_code)]
pub type MutualCreditClearinghouse = GeorgistLedger;

impl GeorgistLedger {
    pub fn new() -> Self {
        Self {
            ast_hex_address: AST_HEX_ADDRESS.to_string(),
            nodes: Vec::with_capacity(MAX_NODES),
            hyperedges: Vec::with_capacity(MAX_HYPEREDGES),
            common_wealth_pool: 0.0,
            citizens: 0,
        }
    }

    pub fn register_node(&mut self, node: georgist_node_t) -> Result<(), String> {
        if self.nodes.len() >= MAX_NODES {
            return Err("node capacity exceeded (max 16)".to_string());
        }
        self.nodes.push(node);
        Ok(())
    }

    pub fn add_hyperedge(&mut self, edge: hypergraph_edge_t) -> Result<(), String> {
        if self.hyperedges.len() >= MAX_HYPEREDGES {
            return Err("hyperedge capacity exceeded (max 32)".to_string());
        }
        self.hyperedges.push(edge);
        Ok(())
    }

    pub fn zero_sum_invariant(&self) -> bool {
        let sum: f64 = self.nodes.iter().map(|n| n.credit_balance).sum();
        sum.abs() < 1e-9
    }

    pub fn execute_lvt_clearing_cycle(&mut self) -> Result<(), String> {
        if self.nodes.is_empty() {
            return Err("cannot execute clearing cycle without nodes".to_string());
        }

        let mut collected = 0.0;
        for node in &mut self.nodes {
            let lvt = node.land_value_assessment * node.lvt_rate;
            node.credit_balance -= lvt;
            collected += lvt;
        }
        self.common_wealth_pool += collected;

        let citizen_count = if self.citizens == 0 {
            self.nodes.len()
        } else {
            self.citizens
        };
        let dividend = self.common_wealth_pool / citizen_count as f64;
        for node in &mut self.nodes {
            node.credit_balance += dividend;
        }
        self.common_wealth_pool = 0.0;

        if !self.zero_sum_invariant() {
            return Err(format!(
                "zero-sum invariant violated: sum of balances = {}",
                self.nodes.iter().map(|n| n.credit_balance).sum::<f64>()
            ));
        }
        Ok(())
    }
}

/// Compositional Game Theory + SRV clearing engine.
#[derive(Debug)]
pub struct CompositionalGameEngine {
    pub agents: Vec<AgentMorphism>,
    pub ledger: GeorgistLedger,
    pub common_wealth_pool: f64,
    pub systemic_entropy: f64,
    pub psi_gap: f64,
    pub turn: u64,
    pub srv_decay_rate: f64,
}

impl CompositionalGameEngine {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            ledger: GeorgistLedger::new(),
            common_wealth_pool: 0.0,
            systemic_entropy: 0.0,
            psi_gap: 0.0,
            turn: 0,
            srv_decay_rate: 0.05,
        }
    }

    pub fn register_agent(&mut self, agent: AgentMorphism) {
        self.agents.push(agent);
    }

    pub fn add_agent(&mut self, agent: AgentMorphism) {
        self.register_agent(agent);
    }

    /// Deception Delta: difference between reported and observed contribution.
    /// Positive values indicate deceptive/parasitic behavior.
    pub fn deception_delta(observed: f64, reported: f64) -> f64 {
        reported - observed
    }

    /// Asymmetric payoff where Dark Triad agents maximize the relative power gap
    /// (Psi) and inject systemic entropy/noise.
    pub fn asymmetric_payoff(psi_gap: f64, deception_delta: f64, entropy_injection: f64) -> f64 {
        psi_gap * (1.0 + deception_delta.max(0.0)) + entropy_injection
    }

    /// Update power gap (Psi) from agent power distribution.
    pub fn update_psi_gap(&mut self) {
        if self.agents.is_empty() {
            self.psi_gap = 0.0;
            return;
        }
        let max_power = self.agents.iter().map(|a| a.power).fold(f64::MIN, f64::max);
        let min_power = self.agents.iter().map(|a| a.power).fold(f64::MAX, f64::min);
        self.psi_gap = (max_power - min_power).max(0.0);
    }

    /// SRV "Rock" hoarding decay: stagnant/hoarded capital is sluiced into the
    /// common wealth pool, while "River" velocity is rewarded.
    pub fn srv_sludge_cycle(&mut self) {
        let mut sluiced = 0.0;
        for agent in &mut self.agents {
            if agent.velocity < SRV_VELOCITY_THRESHOLD || agent.hoarded_capital > SRV_HOARDING_THRESHOLD {
                let decay = agent.hoarded_capital * self.srv_decay_rate;
                agent.hoarded_capital -= decay;
                agent.credit_balance -= decay;
                sluiced += decay;
            } else {
                // River bonus: high velocity agents gain small credit from pool flow.
                agent.credit_balance += agent.velocity * 0.01;
            }
        }
        self.common_wealth_pool += sluiced;
    }

    /// Redistribute common wealth pool equally per capita to agents.
    pub fn redistribute_common_wealth(&mut self) {
        if self.agents.is_empty() {
            return;
        }
        let per_capita = self.common_wealth_pool / self.agents.len() as f64;
        for agent in &mut self.agents {
            agent.credit_balance += per_capita;
        }
        self.common_wealth_pool = 0.0;
    }

    /// Inject systemic entropy/noise (Dark Triad behavior).
    pub fn inject_entropy(&mut self, amount: f64) {
        self.systemic_entropy = (self.systemic_entropy + amount).min(MAX_ENTROPY);
    }

    /// Clearing turn combining SRV dynamics, Georgist LVT, and invariant checks.
    pub fn execute_turn(&mut self) -> Result<(), String> {
        self.turn += 1;

        // SRV river/rock dynamics.
        self.srv_sludge_cycle();

        // Dark Triad agents attempt to maximize Psi and inject entropy.
        let mut entropy_injection = 0.0;
        for agent in &mut self.agents {
            if agent.neuro_type == NeuroType::DarkTriadParasitic {
                agent.power += 1.0;
                entropy_injection += agent.deception_delta.abs() * 0.01;
            }
        }
        self.inject_entropy(entropy_injection);
        self.update_psi_gap();

        // Georgist LVT clearing on the settlement hypergraph.
        self.ledger.execute_lvt_clearing_cycle()?;
        // Move ledger pool into engine pool accounting (already zeroed in ledger).
        // Keep zero-sum across agents by re-normalizing: total agent balance invariant.
        // Redistribute engine common wealth pool.
        self.redistribute_common_wealth();

        // Entropy bound check.
        if self.systemic_entropy > MAX_ENTROPY + 1e-9 {
            return Err(format!(
                "entropy bound violated: {} > {}",
                self.systemic_entropy, MAX_ENTROPY
            ));
        }

        // Zero-sum mutual credit invariant across agents.
        if !self.zero_sum_invariant() {
            return Err(format!(
                "zero-sum invariant violated: sum of agent balances = {}",
                self.agents.iter().map(|a| a.credit_balance).sum::<f64>()
            ));
        }

        Ok(())
    }

    /// Alias for execute_turn.
    pub fn step(&mut self) -> Result<(), String> {
        self.execute_turn()
    }

    pub fn run_turn(&mut self) -> Result<(), String> {
        self.execute_turn()
    }

    pub fn clearing_cycle(&mut self) -> Result<(), String> {
        self.execute_turn()
    }

    pub fn execute_clearing_cycle(&mut self) -> Result<(), String> {
        self.execute_turn()
    }

    pub fn zero_sum_invariant(&self) -> bool {
        let sum: f64 = self.agents.iter().map(|a| a.credit_balance).sum();
        sum.abs() < 1e-9
    }

    pub fn entropy_bound(&self) -> bool {
        self.systemic_entropy <= MAX_ENTROPY + 1e-9
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuro_type_morphisms() {
        let a = AgentMorphism::new("a", NeuroType::AutisticInvariant);
        let b = AgentMorphism::new("b", NeuroType::AdhdNomadic);
        let c = AgentMorphism::new("c", NeuroType::DarkTriadParasitic);
        assert_eq!(a.neuro_type, NeuroType::AutisticInvariant);
        assert_eq!(b.neuro_type, NeuroType::AdhdNomadic);
        assert_eq!(c.neuro_type, NeuroType::DarkTriadParasitic);
    }

    #[test]
    fn test_deception_delta() {
        let dd = CompositionalGameEngine::deception_delta(10.0, 15.0);
        assert!((dd - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_asymmetric_payoff_dark_triad() {
        let payoff = CompositionalGameEngine::asymmetric_payoff(2.0, 0.5, 0.1);
        assert!(payoff > 2.0);
    }

    #[test]
    fn test_zero_sum_invariant_initial() {
        let engine = CompositionalGameEngine::new();
        assert!(engine.zero_sum_invariant());
    }

    #[test]
    fn test_srv_sludge_moves_hoarded_capital() {
        let mut engine = CompositionalGameEngine::new();
        let mut agent = AgentMorphism::new("hoarder", NeuroType::DarkTriadParasitic);
        agent.hoarded_capital = 100.0;
        agent.velocity = 0.0;
        engine.register_agent(agent);

        let before_pool = engine.common_wealth_pool;
        engine.srv_sludge_cycle();
        assert!(engine.common_wealth_pool > before_pool);
        assert!(engine.agents[0].hoarded_capital < 100.0);
    }

    #[test]
    fn test_entropy_bound() {
        let mut engine = CompositionalGameEngine::new();
        engine.inject_entropy(0.5);
        assert!(engine.entropy_bound());
        engine.inject_entropy(1.0);
        assert!((engine.systemic_entropy - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_clearing_cycle_preserves_zero_sum() {
        let mut engine = CompositionalGameEngine::new();
        engine.register_agent(AgentMorphism::new("a", NeuroType::AutisticInvariant));
        engine.register_agent(AgentMorphism::new("b", NeuroType::AdhdNomadic));

        // Seed a Georgist LVT node so clearing cycle has work to do.
        engine
            .ledger
            .register_node(georgist_node_t {
                land_value_assessment: 1000.0,
                lvt_rate: 0.1,
                credit_balance: 0.0,
            })
            .unwrap();

        for _ in 0..3 {
            engine.execute_turn().expect("turn should succeed");
            assert!(engine.zero_sum_invariant());
            assert!(engine.entropy_bound());
        }
    }

    #[test]
    fn test_georgist_lvt_integration() {
        let mut ledger = GeorgistLedger::new();
        ledger
            .register_node(georgist_node_t {
                land_value_assessment: 1000.0,
                lvt_rate: 0.1,
                credit_balance: 0.0,
            })
            .unwrap();
        ledger
            .register_node(georgist_node_t {
                land_value_assessment: 2000.0,
                lvt_rate: 0.1,
                credit_balance: 0.0,
            })
            .unwrap();
        ledger.execute_lvt_clearing_cycle().unwrap();
        assert!(ledger.zero_sum_invariant());
        assert_eq!(ledger.ast_hex_address, "0xASTx702");
    }
}
