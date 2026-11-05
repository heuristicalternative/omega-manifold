#![allow(dead_code)]

use std::collections::HashMap;

pub struct Plan9Vfs {
    node_space: String,
    files: HashMap<String, String>,
}

impl Plan9Vfs {
    pub fn new(node_space: &str) -> Self {
        let mut files = HashMap::new();
        files.insert("/manifold/state".to_string(), "active_inference_idle".to_string());
        files.insert("/manifold/coherence".to_string(), "1.0".to_string());

        Self {
            node_space: node_space.to_string(),
            files,
        }
    }

    pub fn read_file(&self, path: &str) -> Result<String, String> {
        self.files.get(path)
            .cloned()
            .ok_or_else(|| format!("File not found in Plan9 namespace: {}", path))
    }

    pub fn write_file(&mut self, path: &str, content: &str) -> Result<(), String> {
        self.files.insert(path.to_string(), content.to_string());
        Ok(())
    }
}
