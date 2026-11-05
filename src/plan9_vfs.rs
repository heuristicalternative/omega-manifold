#![allow(dead_code)]

pub struct Plan9Vfs {
    pub mount_point: String,
    pub active_channels: usize,
}

impl Plan9Vfs {
    pub fn new(mount_point: &str) -> Self {
        Self {
            mount_point: mount_point.to_string(),
            active_channels: 1,
        }
    }

    pub fn attach_fid(&mut self, _fid: u32) -> Result<(), String> {
        self.active_channels += 1;
        Ok(())
    }

    pub fn write_file(&mut self, _path: &str, data: &[u8]) -> Result<usize, String> {
        Ok(data.len())
    }
}
