use std::ffi::CString;
use std::ptr;
use libc::{shm_open, mmap, munmap, O_RDONLY, PROT_READ, MAP_SHARED};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ShmTelemetryRaw {
    pub sequence_epoch: u64,
    pub active_monad: [u8; 32],
    pub ternary_gate_status: i8,
    pub negentropy_flux: f64,
    pub is_mesh_interlocked: bool,
}

#[derive(Debug, Clone)]
pub struct ShmTelemetry {
    pub sequence_epoch: u64,
    pub active_monad: [u8; 32],
    pub ternary_gate_status: i8,
    pub negentropy_flux: f64,
    pub is_mesh_interlocked: bool,
}

pub fn read_horizon116_shm() -> Result<ShmTelemetry, String> {
    let shm_name = CString::new("/omega_shm_ring_horizon116").unwrap();
    
    unsafe {
        let fd = shm_open(shm_name.as_ptr(), O_RDONLY, 0666);
        if fd == -1 {
            return Err("Failed to open /dev/shm segment. Run horizon116_bridge first.".to_string());
        }

        let ptr = mmap(
            ptr::null_mut(),
            std::mem::size_of::<ShmTelemetryRaw>(),
            PROT_READ,
            MAP_SHARED,
            fd,
            0,
        );

        if ptr == libc::MAP_FAILED {
            return Err("Failed to map shared memory segment.".to_string());
        }

        // Safely read unaligned memory into a local raw container
        let raw = ptr::read_unaligned(ptr as *const ShmTelemetryRaw);
        
        munmap(ptr, std::mem::size_of::<ShmTelemetryRaw>());
        libc::close(fd);

        // Map into standard aligned struct for safe downstream usage
        Ok(ShmTelemetry {
            sequence_epoch: raw.sequence_epoch,
            active_monad: raw.active_monad,
            ternary_gate_status: raw.ternary_gate_status,
            negentropy_flux: raw.negentropy_flux,
            is_mesh_interlocked: raw.is_mesh_interlocked,
        })
    }
}
