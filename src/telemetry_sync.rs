pub struct TelemetryRingBuffer {
    pub capacity: usize,
    pub events: Vec<(u64, String)>,
}

impl TelemetryRingBuffer {
    pub fn new(_capacity: usize) -> Self {
        Self {
            capacity: 32,
            events: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            events: Vec::new(),
        }
    }

    pub fn push_event(&mut self, timestamp: u64, hash: String) -> Result<(), String> {
        self.events.push((timestamp, hash));
        Ok(())
    }

    pub fn sync_to_negentropy(&mut self, _sync_node: &mut crate::negentropy::NegentropySyncNode) -> Result<(), String> {
        Ok(())
    }
}
