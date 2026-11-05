#![allow(dead_code)]
// Vault archiver for complete Omega Manifold session state.

use sha2::{Digest, Sha256};
use std::fs;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VaultArchiver {
    pub baseline: String,
    pub ast_hex: String,
    pub metric_tensors: Vec<f64>,
    pub ast_coords: Vec<(i32, i32, i32)>,
    pub fingerprints: Vec<String>,
}

impl VaultArchiver {
    pub fn new() -> Self {
        Self {
            baseline: "G_Universal_Baseline".to_string(),
            ast_hex: "0xASTx702".to_string(),
            metric_tensors: Vec::new(),
            ast_coords: Vec::new(),
            fingerprints: Vec::new(),
        }
    }

    fn canonical_payload(&self) -> String {
        let tensors: Vec<String> = self.metric_tensors.iter().map(|v| format!("{:.17}", v)).collect();
        let coords: Vec<String> = self
            .ast_coords
            .iter()
            .map(|(x, y, z)| format!("[{},{},{}]", x, y, z))
            .collect();
        format!(
            "baseline={}\nast_hex={}\ntensors={}\ncoords={}\nfingerprints={}",
            self.baseline,
            self.ast_hex,
            tensors.join(","),
            coords.join(","),
            self.fingerprints.join(",")
        )
    }

    pub fn compute_checksum(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.canonical_payload().as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn export_vault_archive(&self, path: &str) -> Result<(), String> {
        let checksum = self.compute_checksum();
        let content = format!(
            "{{\n  \"baseline\": \"{}\",\n  \"ast_hex\": \"{}\",\n  \"metric_tensors\": [{}],\n  \"ast_coords\": [{}],\n  \"fingerprints\": [{}],\n  \"checksum\": \"{}\"\n}}\n",
            self.baseline,
            self.ast_hex,
            self.metric_tensors
                .iter()
                .map(|v| format!("{:.17}", v))
                .collect::<Vec<_>>()
                .join(", "),
            self.ast_coords
                .iter()
                .map(|(x, y, z)| format!("[{},{},{}]", x, y, z))
                .collect::<Vec<_>>()
                .join(", "),
            self.fingerprints
                .iter()
                .map(|f| format!("\"{}\"", f))
                .collect::<Vec<_>>()
                .join(", "),
            checksum
        );

        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn reconcile_with_gdrive(
        &self,
        manifest: &crate::gdrive_sync::GDriveSyncManifest,
        token: &str,
    ) -> Result<bool, String> {
        manifest.evaluate_sheaf_cohomology(token)
    }

    pub fn verify_archive_integrity(&self, path: &str) -> Result<bool, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;

        // Extract stored checksum.
        let stored_checksum = content
            .lines()
            .find_map(|line| {
                let trimmed = line.trim().trim_end_matches(',');
                if trimmed.starts_with("\"checksum\"") {
                    trimmed.split(':').nth(1).map(|s| s.trim().trim_matches('"').to_string())
                } else {
                    None
                }
            })
            .ok_or_else(|| "checksum field not found".to_string())?;

        // Parse fields from the archived file.
        let baseline = content
            .lines()
            .find_map(|line| {
                let trimmed = line.trim().trim_end_matches(',');
                if trimmed.starts_with("\"baseline\"") {
                    trimmed.split(':').nth(1).map(|s| s.trim().trim_matches('"').to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let ast_hex = content
            .lines()
            .find_map(|line| {
                let trimmed = line.trim().trim_end_matches(',');
                if trimmed.starts_with("\"ast_hex\"") {
                    trimmed.split(':').nth(1).map(|s| s.trim().trim_matches('"').to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        // Parse metric_tensors array.
        let tensors_str = content
            .lines()
            .find_map(|line| {
                let trimmed = line.trim().trim_end_matches(',');
                if trimmed.starts_with("\"metric_tensors\"") {
                    let start = trimmed.find('[')?;
                    let end = trimmed.rfind(']')?;
                    Some(trimmed[start + 1..end].to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let metric_tensors: Vec<f64> = if tensors_str.trim().is_empty() {
            Vec::new()
        } else {
            tensors_str
                .split(',')
                .filter_map(|s| s.trim().parse::<f64>().ok())
                .collect()
        };

        // Parse ast_coords array of [x,y,z].
        let coords_str = content
            .lines()
            .find_map(|line| {
                let trimmed = line.trim().trim_end_matches(',');
                if trimmed.starts_with("\"ast_coords\"") {
                    let start = trimmed.find('[')?;
                    let end = trimmed.rfind(']')?;
                    Some(trimmed[start + 1..end].to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let mut ast_coords = Vec::new();
        for part in coords_str.split("],") {
            let cleaned = part.replace('[', "").replace(']', "");
            let nums: Vec<i32> = cleaned
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            if nums.len() == 3 {
                ast_coords.push((nums[0], nums[1], nums[2]));
            }
        }

        // Parse fingerprints array.
        let fp_str = content
            .lines()
            .find_map(|line| {
                let trimmed = line.trim().trim_end_matches(',');
                if trimmed.starts_with("\"fingerprints\"") {
                    let start = trimmed.find('[')?;
                    let end = trimmed.rfind(']')?;
                    Some(trimmed[start + 1..end].to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let fingerprints: Vec<String> = if fp_str.trim().is_empty() {
            Vec::new()
        } else {
            fp_str
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        // Reconstruct an archiver from parsed file data and recompute checksum.
        let parsed = VaultArchiver {
            baseline,
            ast_hex,
            metric_tensors,
            ast_coords,
            fingerprints,
        };
        let recomputed = parsed.compute_checksum();

        Ok(stored_checksum == recomputed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_loss_vault_serialization() {
        let mut archiver = VaultArchiver::new();
        archiver.metric_tensors = vec![1.0, 2.0, 3.0];
        archiver.ast_coords = vec![(1, 2, 3), (4, 5, 6)];
        archiver.fingerprints = vec!["fp1".to_string(), "fp2".to_string()];

        let path = "/tmp/opencode/vault_archive_test.json";
        archiver.export_vault_archive(path).expect("export should succeed");
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("\"baseline\": \"G_Universal_Baseline\""));
        assert!(content.contains("\"ast_hex\": \"0xASTx702\""));
        assert!(archiver.verify_archive_integrity(path).unwrap());

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_integrity_verification_tamper() {
        let archiver = VaultArchiver::new();
        let path = "/tmp/opencode/vault_archive_tamper.json";
        archiver.export_vault_archive(path).unwrap();

        // Tamper with the file.
        let mut content = std::fs::read_to_string(path).unwrap();
        content = content.replace("G_Universal_Baseline", "Tampered");
        std::fs::write(path, content).unwrap();

        // Checksum no longer matches the original archiver.
        assert!(!archiver.verify_archive_integrity(path).unwrap());

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_export_creates_parent_dir() {
        let archiver = VaultArchiver::new();
        let path = "/tmp/opencode/vault/nested/archive.json";
        archiver.export_vault_archive(path).unwrap();
        assert!(std::path::Path::new(path).exists());
        let _ = std::fs::remove_file(path);
    }
}
