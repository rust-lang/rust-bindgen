#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type mbedtls_mpi_uint = ::std::os::raw::c_uint;
/// \brief          MPI structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mbedtls_mpi {
    ///<  integer sign
    pub s: ::std::os::raw::c_int,
    ///<  total # of limbs
    pub n: ::std::os::raw::c_ulong,
    ///<  pointer to limbs
    pub p: *mut mbedtls_mpi_uint,
}
impl Default for mbedtls_mpi {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
