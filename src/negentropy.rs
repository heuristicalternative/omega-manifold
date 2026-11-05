#![allow(dead_code)]

use std::collections::BTreeMap;

pub struct NegentropySyncNode {
    node_id: String,
    state_vector: BTreeMap<u64, String>,
}

impl NegentropySyncNode {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            state_vector: BTreeMap::new(),
        }
    }

    pub fn insert_entry(&mut self, timestamp: u64, state_hash: String) {
        self.state_vector.insert(timestamp, state_hash);
    }

    pub fn generate_range_proof(&self, start_time: u64, end_time: u64) -> Vec<(u64, String)> {
        self.state_vector
            .range(start_time..=end_time)
            .map(|(&k, v)| (k, v.clone()))
            .collect()
    }

    pub fn reconcile(&mut self, peer_entries: &[(u64, String)]) -> usize {
        let mut reconciled_count = 0;
        for (timestamp, hash) in peer_entries {
            let entry = self.state_vector.entry(*timestamp).or_insert_with(|| {
                reconciled_count += 1;
                hash.clone()
            });
            if entry != hash {
                *entry = hash.clone();
                reconciled_count += 1;
            }
        }
        reconciled_count
    }
}
