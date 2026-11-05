#![allow(dead_code)]

pub struct RiemannianManifold {
    dimension: usize,
    metric_tensor: Vec<Vec<f64>>,
}

impl RiemannianManifold {
    pub fn new(dimension: usize, metric_tensor: Vec<Vec<f64>>) -> Self {
        Self {
            dimension,
            metric_tensor,
        }
    }

    pub fn geodesic_distance(&self, p1: &[f64], p2: &[f64]) -> Result<f64, String> {
        if p1.len() != self.dimension || p2.len() != self.dimension {
            return Err("Point dimension mismatch with manifold topology".to_string());
        }
        let mut dist_sq = 0.0;
        for i in 0..self.dimension {
            let diff = p1[i] - p2[i];
            for j in 0..self.dimension {
                dist_sq += diff * self.metric_tensor[i][j] * diff;
            }
        }
        Ok(dist_sq.max(0.0).sqrt())
    }
}
