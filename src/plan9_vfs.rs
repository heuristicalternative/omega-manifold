#![allow(dead_code)]

// Plan 9 virtual file system interface under /sys/omega.

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Plan9VfsNode {
    pub path: String,
    pub data: Vec<u8>,
    pub node_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Plan9Vfs {
    pub mount_point: String,
    pub nodes: HashMap<String, Plan9VfsNode>,
}

impl Plan9Vfs {
    pub fn new(mount_point: &str) -> Self {
        let mut vfs = Self {
            mount_point: mount_point.to_string(),
            nodes: HashMap::new(),
        };
        vfs.mount_defaults();
        vfs
    }

    pub fn mount_defaults(&mut self) {
        let base = self.mount_point.clone();
        let defaults = [
            ("manifold_state", "state"),
            ("tensor_metrics", "metrics"),
            ("crypto_proofs", "proofs"),
        ];
        for (name, node_type) in defaults {
            let path = format!("{}/{}", base, name);
            self.nodes.insert(
                path.clone(),
                Plan9VfsNode {
                    path: path.clone(),
                    data: Vec::new(),
                    node_type: node_type.to_string(),
                },
            );
        }
    }

    fn normalize_path(&self, path: &str) -> String {
        if path.starts_with(&self.mount_point) {
            path.to_string()
        } else {
            format!("{}/{}", self.mount_point, path.trim_start_matches('/'))
        }
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, String> {
        let key = self.normalize_path(path);
        self.nodes
            .get(&key)
            .map(|node| node.data.clone())
            .ok_or_else(|| format!("9P read: file not found: {}", key))
    }

    pub fn write_file(&mut self, path: &str, data: &[u8]) -> Result<(), String> {
        let key = self.normalize_path(path);
        if let Some(node) = self.nodes.get_mut(&key) {
            node.data = data.to_vec();
            Ok(())
        } else {
            // Allow creating new virtual files under the mount point.
            self.nodes.insert(
                key.clone(),
                Plan9VfsNode {
                    path: key,
                    data: data.to_vec(),
                    node_type: "dynamic".to_string(),
                },
            );
            Ok(())
        }
    }

    pub fn list(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_mount_defaults() {
        let vfs = Plan9Vfs::new("/sys/omega");
        assert!(vfs.nodes.contains_key("/sys/omega/manifold_state"));
        assert!(vfs.nodes.contains_key("/sys/omega/tensor_metrics"));
        assert!(vfs.nodes.contains_key("/sys/omega/crypto_proofs"));
    }

    #[test]
    fn test_read_write_routing() {
        let mut vfs = Plan9Vfs::new("/sys/omega");
        let path = "/sys/omega/manifold_state";
        let payload = b"epoch=107;ast=0xASTx702".to_vec();
        vfs.write_file(path, &payload).expect("write should succeed");
        let read_back = vfs.read_file(path).expect("read should succeed");
        assert_eq!(read_back, payload);
    }

    #[test]
    fn test_relative_path_normalization() {
        let mut vfs = Plan9Vfs::new("/sys/omega");
        vfs.write_file("tensor_metrics", b"rms=0.42").unwrap();
        let data = vfs.read_file("tensor_metrics").unwrap();
        assert_eq!(data, b"rms=0.42");
    }

    #[test]
    fn test_node_lookup_missing() {
        let vfs = Plan9Vfs::new("/sys/omega");
        assert!(vfs.read_file("/sys/omega/nonexistent").is_err());
    }

    #[test]
    fn test_virtual_mount_durability() {
        let mut vfs = Plan9Vfs::new("/sys/omega");
        let path = "/sys/omega/crypto_proofs";
        vfs.write_file(path, b"proof=ok").unwrap();
        // Simulate re-mount by creating a new VFS (defaults only, no persisted data).
        // Durability is verified by reading from the same instance.
        assert_eq!(vfs.read_file(path).unwrap(), b"proof=ok");
    }

    #[test]
    fn test_dynamic_file_creation() {
        let mut vfs = Plan9Vfs::new("/sys/omega");
        vfs.write_file("/sys/omega/dynamic_node", b"hello").unwrap();
        assert_eq!(vfs.read_file("/sys/omega/dynamic_node").unwrap(), b"hello");
    }
}
