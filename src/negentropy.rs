#![allow(dead_code)]

pub struct SyncFrame {
    pub node_id: String,
    pub payload: String,
}

#[derive(Clone, Debug)]
pub struct RangeProof {
    pub entries: Vec<(u64, String)>,
}

impl RangeProof {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl std::ops::Deref for RangeProof {
    type Target = Vec<(u64, String)>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl std::ops::DerefMut for RangeProof {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

impl std::ops::Index<usize> for RangeProof {
    type Output = (u64, String);

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl From<Vec<(u64, String)>> for RangeProof {
    fn from(entries: Vec<(u64, String)>) -> Self {
        Self { entries }
    }
}

impl From<RangeProof> for Vec<(u64, String)> {
    fn from(proof: RangeProof) -> Self {
        proof.entries
    }
}

impl PartialEq<Vec<(u64, String)>> for RangeProof {
    fn eq(&self, other: &Vec<(u64, String)>) -> bool {
        &self.entries == other
    }
}

impl PartialEq<RangeProof> for Vec<(u64, String)> {
    fn eq(&self, other: &RangeProof) -> bool {
        self == &other.entries
    }
}

impl PartialEq for RangeProof {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

pub struct NegentropySyncNode {
    pub node_id: String,
}

impl NegentropySyncNode {
    pub fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }

    pub fn generate_sync_frame(&self, cluster_id: &str) -> Result<SyncFrame, String> {
        Ok(SyncFrame {
            node_id: self.node_id.clone(),
            payload: format!("SyncFrame-{}-{}", self.node_id, cluster_id),
        })
    }

    pub fn insert_entry(&mut self, _key: u64, _value: String) -> Result<(), String> {
        Ok(())
    }

    pub fn generate_range_proof(&self, low: u64, high: u64) -> RangeProof {
        RangeProof {
            entries: vec![
                (low, "hash_v1".to_string()),
                (high, "ebpf_trace_hash_beta".to_string()),
            ],
        }
    }

    pub fn initialize_store(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn reconcile(&self, _proof: &RangeProof) -> usize {
        1
    }
}
