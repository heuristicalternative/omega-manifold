#![allow(dead_code)]

use crate::negentropy::NegentropySyncNode;
use std::collections::VecDeque;

pub struct TelemetryRingBuffer {
    capacity: usize,
    buffer: VecDeque<(u64, String)>,
}

impl TelemetryRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push_event(&mut self, timestamp: u64, event_hash: String) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back((timestamp, event_hash));
    }

    pub fn sync_to_negentropy(&self, node: &mut NegentropySyncNode) {
        for (ts, hash) in &self.buffer {
            node.insert_entry(*ts, hash.clone());
        }
    }
}
