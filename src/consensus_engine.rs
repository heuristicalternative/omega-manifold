use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStateVector {
    pub epoch: u64,
    pub root_hash: String,
    pub validated: bool,
}

impl ConsensusStateVector {
    pub fn new(epoch: u64, root_hash: &str) -> Self {
        Self {
            epoch,
            root_hash: root_hash.to_string(),
            validated: false,
        }
    }

    pub fn quorum_verify(&mut self, signatures_count: usize) -> bool {
        if signatures_count >= 2 {
            self.validated = true;
        }
        self.validated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_quorum() {
        let mut state = ConsensusStateVector::new(1, "0x8254737_omega_root");
        assert!(!state.validated, "State should start unvalidated");
        assert!(state.quorum_verify(3), "Quorum verification must pass with >= 2 signatures");
    }
}
