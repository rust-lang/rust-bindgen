#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(feature = "nightly")]
#![feature(non_exhaustive)]
pub type Planet_ctype = ::std::os::raw::c_uint;
pub const Planet_earth: Planet_ctype = 0;
pub const Planet_mars: Planet_ctype = 1;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Planet {
    earth = 0,
    mars = 1,
}
