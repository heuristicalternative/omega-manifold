use std::ffi::CString;
use std::ptr;
use libc::{shm_open, mmap, munmap, O_CREAT, O_RDWR, PROT_READ, PROT_WRITE, MAP_SHARED};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ShmTelemetryRaw {
    pub sequence_epoch: u64,
    pub active_monad: [u8; 32],
    pub ternary_gate_status: i8,
    pub negentropy_flux: f64,
    pub is_mesh_interlocked: bool,
}

pub fn evolve_and_write_shm(epoch: u64) -> Result<(), String> {
    let shm_name = CString::new("/omega_shm_ring_horizon116").unwrap();
    
    // Rotate monads based on epoch modulo
    let (monad_name, flux) = match epoch % 3 {
        0 => ("Bismut_Lott_Torsion_Monad", 1.61803),
        1 => ("Gromov_Witten_Virtual_Monad", 2.71828),
        _ => ("Borcherds_Automorphic_Monad", 3.14159),
    };

    let mut name_buf = [0u8; 32];
    let bytes = monad_name.as_bytes();
    name_buf[..bytes.len()].copy_from_slice(bytes);

    let raw = ShmTelemetryRaw {
        sequence_epoch: epoch,
        active_monad: name_buf,
        ternary_gate_status: 1,
        negentropy_flux: flux,
        is_mesh_interlocked: true,
    };

    unsafe {
        let fd = shm_open(shm_name.as_ptr(), O_CREAT | O_RDWR, 0666);
        if fd == -1 {
            return Err("Failed to open /dev/shm segment for writing.".to_string());
        }

        if libc::ftruncate(fd, std::mem::size_of::<ShmTelemetryRaw>() as i64) == -1 {
            return Err("Failed to ftruncate shared memory.".to_string());
        }

        let ptr = mmap(
            ptr::null_mut(),
            std::mem::size_of::<ShmTelemetryRaw>(),
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        );

        if ptr == libc::MAP_FAILED {
            return Err("Failed to map shared memory for writing.".to_string());
        }

        ptr::write_unaligned(ptr as *mut ShmTelemetryRaw, raw);

        munmap(ptr, std::mem::size_of::<ShmTelemetryRaw>());
        libc::close(fd);
    }

    Ok(())
}
