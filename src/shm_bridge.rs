#![allow(dead_code)]

use std::ffi::CString;
use std::ptr;
use libc::{shm_open, mmap, munmap, close, O_RDONLY, PROT_READ, MAP_SHARED, MAP_FAILED};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ShmTelemetryRaw {
    pub sequence_epoch: u64,
    pub active_monad: [u8; 32],
    pub ternary_gate_status: i8,
    pub negentropy_flux: f64,
    pub is_mesh_interlocked: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ShmTelemetry {
    pub sequence_epoch: u64,
    pub active_monad: [u8; 32],
    pub ternary_gate_status: i8,
    pub negentropy_flux: f64,
    pub is_mesh_interlocked: bool,
}

pub fn read_horizon116_shm() -> Result<ShmTelemetry, String> {
    let shm_name = CString::new("/omega_shm_ring_horizon116").unwrap();

    unsafe {
        let fd = shm_open(shm_name.as_ptr(), O_RDONLY, 0o666);
        if fd == -1 {
            return Err("Failed to open /dev/shm segment. Run horizon116_bridge first.".to_string());
        }

        let ptr = mmap(
            ptr::null_mut(),
            std::mem::size_of::<ShmTelemetryRaw>(),
            PROT_READ,
            MAP_SHARED,
            fd,
            0,
        );

        if ptr == MAP_FAILED {
            return Err("Failed to map shared memory segment.".to_string());
        }

        let raw = ptr::read_unaligned(ptr as *const ShmTelemetryRaw);

        munmap(ptr, std::mem::size_of::<ShmTelemetryRaw>());
        close(fd);

        Ok(ShmTelemetry {
            sequence_epoch: raw.sequence_epoch,
            active_monad: raw.active_monad,
            ternary_gate_status: raw.ternary_gate_status,
            negentropy_flux: raw.negentropy_flux,
            is_mesh_interlocked: raw.is_mesh_interlocked,
        })
    }
}

// VRAM-to-disk persistent memory substrate for stigmergic traces and telemetry buffers.

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ShmBridge {
    pub segments: Vec<Vec<u8>>,
    pub buffer_size: usize,
}

impl ShmBridge {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            segments: Vec::new(),
            buffer_size,
        }
    }

    pub fn add_segment(&mut self, data: Vec<u8>) -> Result<(), String> {
        if data.len() > self.buffer_size && self.buffer_size > 0 {
            return Err("segment exceeds buffer size".to_string());
        }
        self.segments.push(data);
        Ok(())
    }

    /// Flush all VRAM trace segments to disk. Returns total bytes written.
    pub fn flush_vram_traces(&mut self, filepath: &str) -> Result<usize, String> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filepath).map_err(|e| e.to_string())?;
        // Format: u32 segment count, then for each: u32 len + bytes.
        let count = self.segments.len() as u32;
        file.write_all(&count.to_le_bytes()).map_err(|e| e.to_string())?;
        let mut total = 4usize;
        for seg in &self.segments {
            let len = seg.len() as u32;
            file.write_all(&len.to_le_bytes()).map_err(|e| e.to_string())?;
            file.write_all(seg).map_err(|e| e.to_string())?;
            total += 4 + seg.len();
        }
        file.flush().map_err(|e| e.to_string())?;
        Ok(total)
    }

    /// Load persistent traces from disk into memory segments. Returns total bytes read.
    pub fn load_persistent_traces(&mut self, filepath: &str) -> Result<usize, String> {
        use std::fs;
        let data = fs::read(filepath).map_err(|e| e.to_string())?;
        if data.len() < 4 {
            return Err("invalid trace file: too short".to_string());
        }
        let count = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
        let mut offset = 4usize;
        let mut segments = Vec::new();
        let mut total = 4usize;
        for _ in 0..count {
            if offset + 4 > data.len() {
                return Err("invalid trace file: truncated length".to_string());
            }
            let len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if offset + len > data.len() {
                return Err("invalid trace file: truncated segment".to_string());
            }
            segments.push(data[offset..offset + len].to_vec());
            offset += len;
            total += 4 + len;
        }
        self.segments = segments;
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vram_round_trip() {
        let path = "/tmp/opencode/shm_roundtrip.bin";
        let mut bridge = ShmBridge::new(64);
        bridge.add_segment(vec![1, 2, 3, 4]).unwrap();
        bridge.add_segment(vec![5, 6, 7]).unwrap();

        let written = bridge.flush_vram_traces(path).unwrap();
        assert!(written > 0);

        let mut loaded = ShmBridge::new(64);
        let read = loaded.load_persistent_traces(path).unwrap();
        assert_eq!(written, read);
        assert_eq!(loaded.segments, bridge.segments);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_file_io_error_handling() {
        let mut bridge = ShmBridge::new(64);
        let result = bridge.load_persistent_traces("/tmp/opencode/does_not_exist_shm.bin");
        assert!(result.is_err());
    }

    #[test]
    fn test_memory_segment_alignment() {
        let mut bridge = ShmBridge::new(4);
        assert!(bridge.add_segment(vec![1, 2, 3, 4, 5]).is_err());
        assert!(bridge.add_segment(vec![1, 2, 3, 4]).is_ok());
    }

    #[test]
    fn test_empty_segments_round_trip() {
        let path = "/tmp/opencode/shm_empty.bin";
        let mut bridge = ShmBridge::new(16);
        bridge.flush_vram_traces(path).unwrap();
        let mut loaded = ShmBridge::new(16);
        loaded.load_persistent_traces(path).unwrap();
        assert!(loaded.segments.is_empty());
        let _ = std::fs::remove_file(path);
    }
}
