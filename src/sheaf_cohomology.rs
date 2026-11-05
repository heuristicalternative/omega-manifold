#![allow(dead_code)]

pub struct SheafSection {
    pub node_id: String,
    pub local_value: f64,
}

pub struct SheafVerifier {
    pub tolerance: f64,
}

impl SheafVerifier {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    pub fn verify_sections(&self, sections: &[SheafSection]) -> Result<bool, String> {
        if sections.is_empty() {
            return Ok(true);
        }
        let baseline = sections[0].local_value;
        for sec in sections {
            if (sec.local_value - baseline).abs() > self.tolerance {
                return Err(format!(
                    "Cohomology obstruction: node {} value {} deviates from baseline {} beyond tolerance {}",
                    sec.node_id, sec.local_value, baseline, self.tolerance
                ));
            }
        }
        Ok(true)
    }
}
