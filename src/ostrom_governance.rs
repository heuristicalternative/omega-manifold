#![allow(dead_code)]

// Ostrom's polycentric governance principles for Common-Pool Resource (CPR) management.

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommonPoolResource {
    pub resource_id: String,
    pub capacity: f64,
    pub available: f64,
    pub quotas: HashMap<String, f64>,
    pub monitoring_boundary: f64,
    pub graduated_sanctions: HashMap<String, u32>,
    pub extraction_log: HashMap<String, f64>,
}

impl CommonPoolResource {
    pub fn new(resource_id: &str, capacity: f64, monitoring_boundary: f64) -> Self {
        Self {
            resource_id: resource_id.to_string(),
            capacity,
            available: capacity,
            quotas: HashMap::new(),
            monitoring_boundary,
            graduated_sanctions: HashMap::new(),
            extraction_log: HashMap::new(),
        }
    }

    pub fn set_quota(&mut self, node_id: &str, quota: f64) {
        self.quotas.insert(node_id.to_string(), quota);
    }

    /// Enforce proportional equivalence, monitoring boundaries, and graduated sanctions.
    pub fn evaluate_extraction_claim(
        &mut self,
        node_id: &str,
        requested_amount: f64,
    ) -> Result<(), String> {
        if requested_amount < 0.0 {
            return Err("requested amount must be non-negative".to_string());
        }

        let quota = self.quotas.get(node_id).copied().unwrap_or(0.0);
        if requested_amount > quota {
            let sanctions = self.graduated_sanctions.entry(node_id.to_string()).or_insert(0);
            *sanctions += 1;
            return Err(format!(
                "over-extraction: {} requested {:.4} exceeds quota {:.4} (sanction level {})",
                node_id, requested_amount, quota, sanctions
            ));
        }

        if requested_amount > self.available {
            return Err(format!(
                "monitoring boundary violated: requested {:.4} exceeds available {:.4}",
                requested_amount, self.available
            ));
        }

        if requested_amount > self.monitoring_boundary {
            let sanctions = self.graduated_sanctions.entry(node_id.to_string()).or_insert(0);
            *sanctions += 1;
            return Err(format!(
                "monitoring boundary exceeded: {:.4} > {:.4} (sanction level {})",
                requested_amount, self.monitoring_boundary, sanctions
            ));
        }

        // Proportional equivalence: accept extraction and update state.
        self.available -= requested_amount;
        let entry = self.extraction_log.entry(node_id.to_string()).or_insert(0.0);
        *entry += requested_amount;
        Ok(())
    }

    pub fn sanction_level(&self, node_id: &str) -> u32 {
        self.graduated_sanctions.get(node_id).copied().unwrap_or(0)
    }

    pub fn total_extracted(&self, node_id: &str) -> f64 {
        self.extraction_log.get(node_id).copied().unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_extraction() {
        let mut cpr = CommonPoolResource::new("water", 100.0, 50.0);
        cpr.set_quota("node_a", 30.0);
        assert!(cpr.evaluate_extraction_claim("node_a", 20.0).is_ok());
        assert!((cpr.available - 80.0).abs() < 1e-9);
        assert!((cpr.total_extracted("node_a") - 20.0).abs() < 1e-9);
    }

    #[test]
    fn test_over_extraction_triggers_sanction() {
        let mut cpr = CommonPoolResource::new("water", 100.0, 50.0);
        cpr.set_quota("node_b", 10.0);
        let result = cpr.evaluate_extraction_claim("node_b", 20.0);
        assert!(result.is_err());
        assert_eq!(cpr.sanction_level("node_b"), 1);

        // Second violation increases sanction level.
        let _ = cpr.evaluate_extraction_claim("node_b", 15.0);
        assert_eq!(cpr.sanction_level("node_b"), 2);
    }

    #[test]
    fn test_monitoring_boundary_check() {
        let mut cpr = CommonPoolResource::new("water", 100.0, 10.0);
        cpr.set_quota("node_c", 50.0);
        let result = cpr.evaluate_extraction_claim("node_c", 20.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("monitoring boundary"));
    }

    #[test]
    fn test_available_capacity_check() {
        let mut cpr = CommonPoolResource::new("water", 10.0, 10.0);
        cpr.set_quota("node_d", 10.0);
        let _ = cpr.evaluate_extraction_claim("node_d", 10.0);
        // Now available is 0; further extraction fails on availability.
        cpr.set_quota("node_d", 5.0);
        let result = cpr.evaluate_extraction_claim("node_d", 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_quota_rejects() {
        let mut cpr = CommonPoolResource::new("water", 100.0, 50.0);
        // No quota set -> 0
        let result = cpr.evaluate_extraction_claim("unknown", 1.0);
        assert!(result.is_err());
    }
}
