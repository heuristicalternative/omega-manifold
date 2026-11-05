#![allow(dead_code)]

// Memento semantic state caching and RTK token-reduction compression filters.

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MementoCache {
    pub max_tokens: usize,
    pub entries: HashMap<String, Vec<u8>>,
}

impl MementoCache {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            entries: HashMap::new(),
        }
    }

    /// Semantic fingerprint hash of a raw telemetry stream.
    pub fn fingerprint(&self, raw_telemetry: &[f32]) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &value in raw_telemetry {
            let bytes = value.to_le_bytes();
            for b in bytes {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
        format!("{:016x}", hash)
    }

    /// Compress a raw telemetry stream to at most `max_tokens` f32 values,
    /// preserving structural invariants via a leading fingerprint.
    pub fn compress_context(
        &self,
        raw_telemetry: &[f32],
        max_tokens: usize,
    ) -> Result<Vec<u8>, String> {
        if max_tokens == 0 {
            return Err("max_tokens must be greater than zero".to_string());
        }
        if raw_telemetry.is_empty() {
            return Err("raw telemetry must not be empty".to_string());
        }

        let stride = if raw_telemetry.len() <= max_tokens {
            1
        } else {
            (raw_telemetry.len() + max_tokens - 1) / max_tokens
        };

        let mut compressed: Vec<f32> = raw_telemetry.iter().step_by(stride).copied().collect();
        if compressed.len() > max_tokens {
            compressed.truncate(max_tokens);
        }
        if compressed.is_empty() {
            compressed.push(raw_telemetry[0]);
        }

        // Encode: [fingerprint len (u32)] [fingerprint bytes] [count (u32)] [f32 bytes...]
        let fp = self.fingerprint(raw_telemetry);
        let fp_bytes = fp.as_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&(fp_bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(fp_bytes);
        out.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        for v in &compressed {
            out.extend_from_slice(&v.to_le_bytes());
        }
        Ok(out)
    }

    pub fn cache(&mut self, raw_telemetry: &[f32], max_tokens: usize) -> Result<String, String> {
        let compressed = self.compress_context(raw_telemetry, max_tokens)?;
        let fp = self.fingerprint(raw_telemetry);
        self.entries.insert(fp.clone(), compressed);
        Ok(fp)
    }

    pub fn retrieve(&self, fingerprint: &str) -> Option<Vec<u8>> {
        self.entries.get(fingerprint).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_compression_ratio() {
        let cache = MementoCache::new(4);
        let raw: Vec<f32> = (0..100).map(|i| i as f32).collect();
        let compressed = cache.compress_context(&raw, 4).unwrap();
        // Should encode at most 4 f32 values plus header.
        assert!(compressed.len() < raw.len() * 4);
    }

    #[test]
    fn test_threshold_enforcement() {
        let cache = MementoCache::new(4);
        let raw: Vec<f32> = (0..100).map(|i| i as f32).collect();
        let compressed = cache.compress_context(&raw, 4).unwrap();
        // Decode count.
        let mut idx = 4 + u32::from_le_bytes(compressed[0..4].try_into().unwrap()) as usize;
        let count = u32::from_le_bytes(compressed[idx..idx + 4].try_into().unwrap()) as usize;
        _ = idx; idx += 4;
        assert!(count <= 4);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_round_trip_cache_retrieval() {
        let mut cache = MementoCache::new(8);
        let raw: Vec<f32> = (0..20).map(|i| i as f32 * 0.5).collect();
        let fp = cache.cache(&raw, 8).unwrap();
        let retrieved = cache.retrieve(&fp).expect("should retrieve");
        assert!(!retrieved.is_empty());
        // Fingerprint should remain stable.
        assert_eq!(cache.fingerprint(&raw), fp);
    }

    #[test]
    fn test_max_tokens_zero_errors() {
        let cache = MementoCache::new(4);
        let raw = vec![1.0f32, 2.0];
        assert!(cache.compress_context(&raw, 0).is_err());
    }

    #[test]
    fn test_empty_input_errors() {
        let cache = MementoCache::new(4);
        assert!(cache.compress_context(&[], 4).is_err());
    }
}
