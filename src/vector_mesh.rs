#![allow(dead_code)]
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchQuery {
    pub vector: Vec<f32>,
    pub limit: usize,
}

pub struct VectorMesh {
    pub endpoint: String,
    client: reqwest::Client,
}

impl VectorMesh {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn query_mesh(&self, query: &SearchQuery) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/collections/points/search", self.endpoint);
        self.client.post(&url)
            .json(query)
            .send()
            .await
    }
}
