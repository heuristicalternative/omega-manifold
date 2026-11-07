pub struct EbpfProbe;

impl EbpfProbe {
    pub fn new() -> Self {
        Self
    }

    pub fn is_available(&self) -> bool {
        true
    }
}
