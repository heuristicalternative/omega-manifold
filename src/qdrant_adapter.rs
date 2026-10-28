#![allow(dead_code)]

// In-process edge vector storage and retrieval substrate for Omega Manifold states.

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstHexCoordinate {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub node_id: String,
    pub symbol_path: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct OmegaStatePayload {
    pub point_id: String,
    pub dimension: usize,
    pub baseline_status: String,
    pub ast_hex: AstHexCoordinate,
    pub logic_set_tags: Vec<String>,
    pub timestamp: u64,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct OmegaQdrantAdapter {
    pub collection: String,
    pub points: Vec<OmegaStatePayload>,
}

impl OmegaQdrantAdapter {
    pub fn new(collection: &str) -> Self {
        Self {
            collection: collection.to_string(),
            points: Vec::new(),
        }
    }

    pub fn store_state_point(&mut self, payload: OmegaStatePayload) -> Result<(), String> {
        if payload.point_id.trim().is_empty() {
            return Err("point_id must not be empty".to_string());
        }
        if payload.baseline_status != "G_Universal_Baseline" {
            return Err("baseline_status must be G_Universal_Baseline".to_string());
        }
        self.points.push(payload);
        Ok(())
    }

    pub fn query_baseline_states(&self, baseline: &str) -> Vec<OmegaStatePayload> {
        self.points
            .iter()
            .filter(|p| p.baseline_status == baseline)
            .cloned()
            .collect()
    }

    pub fn query_by_tag(&self, tag: &str) -> Vec<OmegaStatePayload> {
        self.points
            .iter()
            .filter(|p| p.logic_set_tags.iter().any(|t| t == tag))
            .cloned()
            .collect()
    }

    pub fn count(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_point(id: &str, x: i32, y: i32, z: i32) -> OmegaStatePayload {
        OmegaStatePayload {
            point_id: id.to_string(),
            dimension: 35,
            baseline_status: "G_Universal_Baseline".to_string(),
            ast_hex: AstHexCoordinate {
                x,
                y,
                z,
                node_id: "melusine_node_01".to_string(),
                symbol_path: "0xASTx702".to_string(),
            },
            logic_set_tags: vec!["omega".to_string(), "sheaf".to_string()],
            timestamp: 107,
        }
    }

    #[test]
    fn test_store_state_point() {
        let mut adapter = OmegaQdrantAdapter::new("omega_states");
        assert!(adapter.store_state_point(sample_point("p1", 1, 2, 3)).is_ok());
        assert_eq!(adapter.count(), 1);
    }

    #[test]
    fn test_query_baseline_states() {
        let mut adapter = OmegaQdrantAdapter::new("omega_states");
        adapter.store_state_point(sample_point("p1", 1, 2, 3)).unwrap();

        let baseline = adapter.query_baseline_states("G_Universal_Baseline");
        assert_eq!(baseline.len(), 1);
        assert_eq!(baseline[0].point_id, "p1");

        let none = adapter.query_baseline_states("Other");
        assert!(none.is_empty());
    }

    #[test]
    fn test_query_by_tag() {
        let mut adapter = OmegaQdrantAdapter::new("omega_states");
        adapter.store_state_point(sample_point("p1", 1, 2, 3)).unwrap();
        let results = adapter.query_by_tag("sheaf");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_invalid_baseline_rejected() {
        let mut adapter = OmegaQdrantAdapter::new("omega_states");
        let mut bad = sample_point("p1", 1, 2, 3);
        bad.baseline_status = "Bad".to_string();
        assert!(adapter.store_state_point(bad).is_err());
    }

    #[test]
    fn test_empty_point_id_rejected() {
        let mut adapter = OmegaQdrantAdapter::new("omega_states");
        let mut bad = sample_point("", 1, 2, 3);
        bad.point_id = "".to_string();
        assert!(adapter.store_state_point(bad).is_err());
    }
}
