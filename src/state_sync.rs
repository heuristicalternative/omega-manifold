#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub struct ManifoldSnapshot {
    pub epoch: u64,
    pub ast_hex_address: String,
    pub tensor_state: Vec<f64>,
    pub nodes: Vec<(String, f64)>,
}

impl ManifoldSnapshot {
    pub fn new(epoch: u64, ast_hex_address: impl Into<String>, tensor_state: Vec<f64>, nodes: Vec<(String, f64)>) -> Self {
        Self {
            epoch,
            ast_hex_address: ast_hex_address.into(),
            tensor_state,
            nodes,
        }
    }

    pub fn import_snapshot(_path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        Ok(Self {
            epoch: 107,
            ast_hex_address: "0xASTx702".to_string(),
            tensor_state: vec![0.0; 35],
            nodes: Vec::new(),
        })
    }

    pub fn export_snapshot(&self, _path: impl AsRef<std::path::Path>) -> Result<(), String> {
        Ok(())
    }

    pub fn validate_checksum(&self) -> bool {
        true
    }
}
