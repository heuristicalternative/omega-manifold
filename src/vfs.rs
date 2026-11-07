#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct Plan9Vfs {
    pub namespace: String,
    pub active_channels: usize,
    pub mount_point: String,
}

impl Plan9Vfs {
    pub fn new(namespace: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            active_channels: 2,
            mount_point: "/mnt/omega".to_string(),
        }
    }

    pub fn attach_fid(&mut self, _fid: u32) -> Result<(), String> {
        Ok(())
    }

    pub fn read_file(&self, _path: impl AsRef<std::path::Path>) -> Result<String, String> {
        Ok("Melusine-Node FE Bound State Content".to_string())
    }

    pub fn write_file(&self, _path: impl AsRef<std::path::Path>, _content: &str) -> Result<(), String> {
        Ok(())
    }
}
