#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type myVector3 = [f32; 3usize];
extern "C" {
    pub fn modifyVectorFunc(v: *mut f32);
}
