#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C, align(16))]
pub struct __BindgenLongDouble([u8; 16]);
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
extern "C" {
    pub static mut globalValueFloat: __BindgenComplex<f32>;
}
extern "C" {
    pub static mut globalValueDouble: __BindgenComplex<f64>;
}
extern "C" {
    pub static mut globalValueLongDouble: __BindgenComplex<__BindgenLongDouble>;
}
