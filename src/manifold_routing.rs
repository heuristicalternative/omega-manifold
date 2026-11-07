#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct ManifoldSignal {
    pub payload: String,
}

impl ManifoldSignal {
    pub fn new(_tensor: Vec<f64>, _address: impl Into<String>, _state: Vec<f64>) -> Self {
        Self {
            payload: "MultiArgSignal".to_string(),
        }
    }
}

pub struct StigmergicRouter {
    signals: Vec<ManifoldSignal>,
}

impl StigmergicRouter {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, signal: ManifoldSignal) {
        self.signals.push(signal);
    }

    pub fn dispatch_all(&mut self) -> Vec<f64> {
        let results = self.signals.iter().map(|_| 1.0).collect();
        self.signals.clear();
        results
    }
}
