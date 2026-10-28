use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct VaultRecord {
    pub sequence_epoch: u64,
    pub active_monad: [u8; 32],
    pub ternary_gate_status: i8,
    pub negentropy_flux: f64,
    pub checksum: u64,
}

pub fn persist_epoch_to_vault(epoch: u64, monad: &[u8; 32], gate: i8, flux: f64) -> Result<(), String> {
    let vault_path = "omega_vault.hmmf";
    
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(vault_path)
        .map_err(|e| format!("Failed to open HMMF vault file: {}", e))?;

    let record_size = std::mem::size_of::<VaultRecord>() as u64;
    let target_offset = epoch * record_size;

    let checksum = epoch ^ fanotify_checksum_placeholder(flux);

    let record = VaultRecord {
        sequence_epoch: epoch,
        active_monad: *monad,
        ternary_gate_status: gate,
        negentropy_flux: flux,
        checksum,
    };

    file.seek(SeekFrom::Start(target_offset))
        .map_err(|e| format!("Failed to seek in HMMF vault: {}", e))?;

    let ptr = &record as *const VaultRecord as *const u8;
    let bytes = unsafe { std::slice::from_raw_parts(ptr, record_size as usize) };

    file.write_all(bytes)
        .map_err(|e| format!("Failed to write record to HMMF vault: {}", e))?;

    file.flush().map_err(|e| format!("Failed to flush HMMF vault: {}", e))?;

    Ok(())
}

fn fanotify_checksum_placeholder(flux: f64) -> u64 {
    (flux * 1_000_000.0) as u64
}
