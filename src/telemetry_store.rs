use std::fs::{OpenOptions, File};
use std::io::{Write, BufWriter, Result as IoResult};

pub struct TelemetryStore {
    log_path: String,
}

impl TelemetryStore {
    pub fn new(log_path: &str) -> Self {
        Self {
            log_path: log_path.to_string(),
        }
    }

    pub fn append_snapshot(&self, timestamp: u64, trace_hash: &str, weight: f32) -> IoResult<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;
        
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{},{},{}", timestamp, trace_hash, weight)?;
        Ok(())
    }

    pub fn log_path(&self) -> &str {
        &self.log_path
    }
}
