#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type float4 = [f32; 4usize];
pub type float2 = [f32; 2usize];
unsafe extern "C" {
    pub fn foo(a: float2, b: float2) -> float4;
}
