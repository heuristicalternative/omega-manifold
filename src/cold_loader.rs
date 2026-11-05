#![allow(dead_code)]
// C23 Cold Substrate Re-hydration Loader interface (AST Hex: 0x434F4C445F524548594452415445).

use std::os::raw::{c_char, c_int, c_void};

pub const COLD_REHYDRATION_AST_HEX: &[u8] = b"0x434F4C445F524548594452415445";

#[allow(dead_code)]
pub struct ColdRehydrationManager {
    pub library_path: String,
    handle: Option<*mut c_void>,
}

impl ColdRehydrationManager {
    pub fn new() -> Self {
        Self {
            library_path: Self::candidate_paths()
                .into_iter()
                .next()
                .unwrap_or_else(|| "libcold_rehydration_loader.so".to_string()),
            handle: None,
        }
    }

    pub fn candidate_paths() -> Vec<String> {
        vec![
            "/home/dante/omega_manifold/lib/libcold_rehydration_loader.so".to_string(),
            "./libcold_rehydration_loader.so".to_string(),
            "libcold_rehydration_loader.so".to_string(),
        ]
    }

    pub fn load(&mut self) -> Result<(), String> {
        for path in Self::candidate_paths() {
            let c_path = std::ffi::CString::new(path.clone())
                .map_err(|e| e.to_string())?;
            unsafe {
                let handle = libc::dlopen(c_path.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL);
                if !handle.is_null() {
                    self.library_path = path;
                    self.handle = Some(handle);
                    return Ok(());
                }
            }
        }
        Err("cold rehydration shared library not found".to_string())
    }

    pub fn unload(&mut self) {
        if let Some(handle) = self.handle.take() {
            unsafe {
                libc::dlclose(handle);
            }
        }
    }

    pub fn init_rehydration_engine(&self) -> Result<(), String> {
        let handle = self.handle.ok_or_else(|| "library not loaded".to_string())?;
        unsafe {
            let sym = libc::dlsym(handle, b"init_rehydration_engine\0".as_ptr() as *const c_char);
            if sym.is_null() {
                return Err("symbol init_rehydration_engine not found".to_string());
            }
            let func: extern "C" fn() -> c_int =
                std::mem::transmute(sym);
            let rc = func();
            if rc == 0 {
                Ok(())
            } else {
                Err(format!("init_rehydration_engine returned {}", rc))
            }
        }
    }

    pub fn verify_ast_hex_anchor(&self, anchor: &[u8]) -> Result<bool, String> {
        if let Some(handle) = self.handle {
            unsafe {
                let sym = libc::dlsym(handle, b"verify_ast_hex_anchor\0".as_ptr() as *const c_char);
                if !sym.is_null() {
                    let func: extern "C" fn(*const c_char, usize) -> c_int =
                        std::mem::transmute(sym);
                    let c_anchor = std::ffi::CString::new(anchor).map_err(|e| e.to_string())?;
                    let rc = func(c_anchor.as_ptr(), anchor.len());
                    return Ok(rc != 0);
                }
            }
        }

        // Fallback validation when the shared library is unavailable.
        Ok(anchor == COLD_REHYDRATION_AST_HEX)
    }
}

impl Drop for ColdRehydrationManager {
    fn drop(&mut self) {
        self.unload();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_hex_anchor_validation() {
        let manager = ColdRehydrationManager::new();
        assert!(manager
            .verify_ast_hex_anchor(COLD_REHYDRATION_AST_HEX)
            .unwrap());
        assert!(!manager.verify_ast_hex_anchor(b"0xDEADBEEF").unwrap());
    }

    #[test]
    fn test_candidate_paths_include_expected() {
        let paths = ColdRehydrationManager::candidate_paths();
        assert!(paths
            .iter()
            .any(|p| p.contains("libcold_rehydration_loader.so")));
    }

    #[test]
    fn test_missing_library_load_fails_gracefully() {
        let mut manager = ColdRehydrationManager::new();
        // In this environment the C23 shared object is not present.
        let result = manager.load();
        // Either it loads (if present) or fails gracefully.
        if result.is_err() {
            assert!(manager.handle.is_none());
        } else {
            assert!(manager.handle.is_some());
            manager.unload();
        }
    }
}
