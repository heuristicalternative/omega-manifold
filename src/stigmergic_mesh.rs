#![allow(dead_code)]

use std::collections::HashMap;

pub struct StigmergicMesh {
    node_id: String,
    pheromone_trails: HashMap<String, f64>,
}

impl StigmergicMesh {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            pheromone_trails: HashMap::new(),
        }
    }

    pub fn deposit_trace(&mut self, key: &str, intensity: f64) {
        self.pheromone_trails.insert(key.to_string(), intensity);
    }

    pub fn read_trace(&self, key: &str) -> Option<&f64> {
        self.pheromone_trails.get(key)
    }
}
