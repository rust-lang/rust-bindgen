#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, unused_mut, unsafe_code)]
#[inline(always)]
pub unsafe extern "C" fn MEMORY_BARRIER() {
    ::std::arch::asm!("", options(preserves_flags),);
}
