#![allow(dead_code)]

pub struct TelemetryRingBuffer {
    pub capacity: usize,
}

impl TelemetryRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { capacity }
    }
}
