use std::fs::File;
use std::io::{BufRead, BufReader, Result as IoResult};

pub struct ClusterMetricsSummary {
    pub total_events: usize,
    pub average_weight: f32,
    pub latest_timestamp: u64,
}

pub struct ClusterMetricsAggregator {
    log_path: String,
}

impl ClusterMetricsAggregator {
    pub fn new(log_path: &str) -> Self {
        Self {
            log_path: log_path.to_string(),
        }
    }

    pub fn summarize(&self) -> IoResult<ClusterMetricsSummary> {
        let file = File::open(&self.log_path)?;
        let reader = BufReader::new(file);
        
        let mut total_events = 0;
        let mut weight_sum = 0.0f32;
        let mut latest_timestamp = 0u64;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                if let Ok(ts) = parts[0].parse::<u64>() {
                    latest_timestamp = latest_timestamp.max(ts);
                }
                if let Ok(w) = parts[2].parse::<f32>() {
                    weight_sum += w;
                }
                total_events += 1;
            }
        }

        let average_weight = if total_events > 0 {
            weight_sum / total_events as f32
        } else {
            0.0
        };

        Ok(ClusterMetricsSummary {
            total_events,
            average_weight,
            latest_timestamp,
        })
    }
}
