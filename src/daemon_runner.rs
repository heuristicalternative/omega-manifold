#![allow(dead_code)]

pub struct OmegaDaemonRunner;

impl OmegaDaemonRunner {
    pub fn new() -> Self {
        Self
    }

    pub fn run_full_lifecycle(&mut self, _steps: usize) -> Result<(), String> {
        Ok(())
    }
}
