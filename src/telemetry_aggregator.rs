use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryEvent {
    pub timestamp: u64,
    pub trace_hash: String,
    pub manifold_weight: f64,
}

pub struct TelemetryAggregator {
    ring_buffer: Arc<Mutex<VecDeque<TelemetryEvent>>>,
    capacity: usize,
}

impl TelemetryAggregator {
    pub fn new(capacity: usize) -> Self {
        Self {
            ring_buffer: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            capacity,
        }
    }

    pub fn push_event(&self, timestamp: u64, trace_hash: String, manifold_weight: f64) {
        let mut buf = self.ring_buffer.lock().unwrap();
        if buf.len() >= self.capacity {
            buf.pop_front();
        }
        buf.push_back(TelemetryEvent {
            timestamp,
            trace_hash,
            manifold_weight,
        });
    }

    pub fn snapshot_events(&self) -> Vec<TelemetryEvent> {
        let buf = self.ring_buffer.lock().unwrap();
        buf.iter().cloned().collect()
    }
}
