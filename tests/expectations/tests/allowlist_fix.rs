#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub enum Test {}

extern "C" {
    pub fn Servo_Test(a: *mut Test);
}
