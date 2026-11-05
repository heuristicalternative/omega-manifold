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

impl OmegaOrchestrator {
    pub fn new(node_id: &str, expected_states: Vec<f64>, precision: f64, tolerance: f64) -> Self {
        Self {
            node_id: node_id.to_string(),
            agent: ActiveInferenceAgent::new(node_id, expected_states, precision),
            mesh: StigmergicMesh::new(node_id),
            verifier: SheafVerifier::new(tolerance),
            sync_node: NegentropySyncNode::new(node_id),
            gossip: GossipProtocol::new(node_id),
        }
    }

    pub fn step(&mut self, sensory_input: &[f64]) -> Result<f64, String> {
        let fe = self.agent.calculate_free_energy(sensory_input);
        self.mesh.deposit_trace("free_energy", fe);
        
        let sections = vec![SheafSection {
            node_id: self.node_id.clone(),
            local_value: fe,
        }];
        self.verifier.verify_sections(&sections)?;
        
        self.sync_node.insert_entry(100, fe.to_string());
        Ok(fe)
    }
}
