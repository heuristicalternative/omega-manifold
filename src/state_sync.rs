#![allow(dead_code)]

// Topological state serialization and cryptographic snapshot export.

use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct ManifoldSnapshot {
    pub epoch: u64,
    pub ast_hex_address: String,
    pub tensor_state: Vec<f64>,
    pub mutual_credit_balances: Vec<(String, f64)>,
    pub checksum: String,
}

impl ManifoldSnapshot {
    pub fn new(
        epoch: u64,
        ast_hex_address: String,
        tensor_state: Vec<f64>,
        mutual_credit_balances: Vec<(String, f64)>,
    ) -> Self {
        let mut snapshot = Self {
            epoch,
            ast_hex_address,
            tensor_state,
            mutual_credit_balances,
            checksum: String::new(),
        };
        snapshot.checksum = snapshot.compute_checksum();
        snapshot
    }

    fn canonical_payload(&self) -> String {
        let tensor: Vec<String> = self.tensor_state.iter().map(|v| format!("{:.17}", v)).collect();
        let balances: Vec<String> = self
            .mutual_credit_balances
            .iter()
            .map(|(k, v)| format!("{}={:.17}", k, v))
            .collect();
        format!(
            "epoch={}\nast={}\ntensor={}\nbalances={}",
            self.epoch,
            self.ast_hex_address,
            tensor.join(","),
            balances.join(";")
        )
    }

    pub fn compute_checksum(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.canonical_payload().as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn validate_checksum(&self) -> bool {
        self.checksum == self.compute_checksum()
    }

    pub fn verify_zk_proofs(
        &self,
        proofs: &[crate::zero_knowledge::Groth16Proof],
    ) -> Result<bool, std::io::Error> {
        crate::zero_knowledge::Groth16Verifier::new()
            .verify_batched_proofs(proofs)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn export_snapshot(&self, path: &str) -> Result<(), io::Error> {
        let mut file = fs::File::create(path)?;
        writeln!(file, "epoch={}", self.epoch)?;
        writeln!(file, "ast={}", self.ast_hex_address)?;
        let tensor: Vec<String> = self.tensor_state.iter().map(|v| format!("{:.17}", v)).collect();
        writeln!(file, "tensor={}", tensor.join(","))?;
        let balances: Vec<String> = self
            .mutual_credit_balances
            .iter()
            .map(|(k, v)| format!("{}={:.17}", k, v))
            .collect();
        writeln!(file, "balances={}", balances.join(";"))?;
        writeln!(file, "checksum={}", self.checksum)?;
        Ok(())
    }

    pub fn import_snapshot(path: &str) -> Result<Self, io::Error> {
        let content = fs::read_to_string(path)?;
        let mut epoch = 0u64;
        let mut ast = String::new();
        let mut tensor = Vec::new();
        let mut balances = Vec::new();
        let mut checksum = String::new();

        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("epoch=") {
                epoch = rest.trim().parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "invalid epoch")
                })?;
            } else if let Some(rest) = line.strip_prefix("ast=") {
                ast = rest.trim().to_string();
            } else if let Some(rest) = line.strip_prefix("tensor=") {
                if !rest.trim().is_empty() {
                    for part in rest.split(',') {
                        let v: f64 = part.trim().parse().map_err(|_| {
                            io::Error::new(io::ErrorKind::InvalidData, "invalid tensor value")
                        })?;
                        tensor.push(v);
                    }
                }
            } else if let Some(rest) = line.strip_prefix("balances=") {
                if !rest.trim().is_empty() {
                    for part in rest.split(';') {
                        if part.trim().is_empty() {
                            continue;
                        }
                        let mut kv = part.splitn(2, '=');
                        let key = kv
                            .next()
                            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid balance key"))?
                            .to_string();
                        let val: f64 = kv
                            .next()
                            .ok_or_else(|| {
                                io::Error::new(io::ErrorKind::InvalidData, "invalid balance value")
                            })?
                            .trim()
                            .parse()
                            .map_err(|_| {
                                io::Error::new(io::ErrorKind::InvalidData, "invalid balance number")
                            })?;
                        balances.push((key, val));
                    }
                }
            } else if let Some(rest) = line.strip_prefix("checksum=") {
                checksum = rest.trim().to_string();
            }
        }

        let snapshot = Self {
            epoch,
            ast_hex_address: ast,
            tensor_state: tensor,
            mutual_credit_balances: balances,
            checksum,
        };

        if !snapshot.validate_checksum() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "checksum mismatch",
            ));
        }

        Ok(snapshot)
    }
}

pub fn index_snapshot(
    snapshot: &ManifoldSnapshot,
    adapter: &mut crate::qdrant_adapter::OmegaQdrantAdapter,
) {
    let payload = crate::qdrant_adapter::OmegaStatePayload {
        point_id: format!("epoch-{}", snapshot.epoch),
        dimension: snapshot.tensor_state.len(),
        baseline_status: "G_Universal_Baseline".to_string(),
        ast_hex: crate::qdrant_adapter::AstHexCoordinate {
            x: 0,
            y: 0,
            z: 0,
            node_id: "melusine_node_01".to_string(),
            symbol_path: snapshot.ast_hex_address.clone(),
        },
        logic_set_tags: vec!["omega".to_string(), "state_sync".to_string()],
        timestamp: snapshot.epoch,
    };
    let _ = adapter.store_state_point(payload);
}

pub fn validate_zk_proofs(
    proofs: &[crate::zero_knowledge::Groth16Proof],
) -> Result<bool, std::io::Error> {
    crate::zero_knowledge::Groth16Verifier::new()
        .verify_batched_proofs(proofs)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_round_trip_serialization() {
        let snapshot = ManifoldSnapshot::new(
            107,
            "0xASTx702".to_string(),
            vec![1.0, 2.0, 3.0],
            vec![("node_a".to_string(), 10.0), ("node_b".to_string(), -10.0)],
        );

        let path = "/tmp/opencode/snapshot_roundtrip.txt";
        snapshot.export_snapshot(path).expect("export should succeed");

        let imported = ManifoldSnapshot::import_snapshot(path).expect("import should succeed");
        assert_eq!(imported, snapshot);
        assert!(imported.validate_checksum());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_checksum_mismatch_protection() {
        let snapshot = ManifoldSnapshot::new(
            1,
            "0xASTx702".to_string(),
            vec![1.0, 2.0],
            vec![],
        );
        let path = "/tmp/opencode/snapshot_tamper.txt";
        snapshot.export_snapshot(path).expect("export should succeed");

        // Tamper with the file.
        let mut content = fs::read_to_string(path).unwrap();
        content = content.replace("epoch=1", "epoch=2");
        fs::write(path, content).unwrap();

        let result = ManifoldSnapshot::import_snapshot(path);
        assert!(result.is_err());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_checksum_validation() {
        let mut snapshot = ManifoldSnapshot::new(5, "0xASTx702".to_string(), vec![0.5], vec![]);
        assert!(snapshot.validate_checksum());
        snapshot.tensor_state.push(999.0);
        assert!(!snapshot.validate_checksum());
    }
}
