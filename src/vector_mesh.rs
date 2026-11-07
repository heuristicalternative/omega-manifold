#![allow(dead_code)]

pub struct VectorMesh {
    pub endpoint: String,
}

impl VectorMesh {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
        }
    }
}
