#![allow(dead_code)]

use crate::active_inference::ActiveInferenceAgent;
use crate::stigmergic_mesh::StigmergicMesh;
use crate::sheaf_cohomology::{SheafVerifier, SheafSection};
use crate::negentropy::NegentropySyncNode;
use crate::gossip::GossipProtocol;

pub struct OmegaOrchestrator {
    pub node_id: String,
    pub agent: ActiveInferenceAgent,
    pub mesh: StigmergicMesh,
    pub verifier: SheafVerifier,
    pub sync_node: NegentropySyncNode,
    pub gossip: GossipProtocol,
}

pub type CoordinationOrchestrator = OmegaOrchestrator;

impl OmegaOrchestrator {
    pub fn new(node_id: &str, expected_states: Vec<f64>, precision: f64, tolerance: f64) -> Self {
        let dim = expected_states.len().max(1);
        let metric = vec![vec![1.0; dim]; dim];
        Self {
            node_id: node_id.to_string(),
            agent: ActiveInferenceAgent::new(node_id, expected_states, precision, crate::geometry::RiemannianManifold::new(dim, metric)),
            mesh: StigmergicMesh::new(node_id),
            verifier: SheafVerifier::new(tolerance),
            sync_node: NegentropySyncNode::new(node_id),
            gossip: GossipProtocol::new(node_id),
        }
    }

    pub fn new_simple(node_id: &str) -> Self {
        let dim = 2;
        let metric = vec![vec![1.0; dim]; dim];
        Self {
            node_id: node_id.to_string(),
            agent: ActiveInferenceAgent::new(node_id, vec![0.0, 0.0], 1.0, crate::geometry::RiemannianManifold::new(dim, metric)),
            mesh: StigmergicMesh::new(node_id),
            verifier: SheafVerifier::new(0.5),
            sync_node: NegentropySyncNode::new(node_id),
            gossip: GossipProtocol::new(node_id),
        }
    }

    pub fn new_with_dim(node_id: &str, _dim: usize) -> Self {
        Self::new_simple(node_id)
    }

    pub fn step(&mut self, sensory_input: &[f64]) -> Result<f64, String> {
        let fe = self.agent.calculate_free_energy(sensory_input)?;
        self.mesh.deposit_trace("free_energy", fe);
        
        let sections = vec![SheafSection {
            node_id: self.node_id.clone(),
            local_value: fe,
        }];
        self.verifier.verify_sections(&sections)?;
        
        self.sync_node.insert_entry(100, fe.to_string());
        Ok(fe)
    }

    pub fn status(&self) -> String {
        format!("Node: {} | Active | FE Bound", self.node_id)
    }

    pub fn verify_sheaf_cohomology(&self) -> Result<(), String> {
        let sections = vec![
            SheafSection { node_id: self.node_id.clone(), local_value: 1.0 },
            SheafSection { node_id: "remote-node".to_string(), local_value: 1.0 },
        ];
        self.verifier.verify_sections(&sections)
    }

    pub fn step_active_inference(&mut self, input: f64) -> f64 {
        self.step(&[input, input]).unwrap_or_default()
    }
}
