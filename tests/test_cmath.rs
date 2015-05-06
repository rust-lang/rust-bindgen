// Unused until we can generate code for tests

#[allow(dead_code, non_camel_case_types)]
pub mod ffi { bindgen!("/usr/include/math.h", link = "m"); }

#[test]
fn floor_is_bound_and_callable() {
    unsafe {
        assert_eq!(ffi::floor( 2.7),  2.0);
        assert_eq!(ffi::floor(-2.7), -3.0);
        assert_eq!(ffi::floor(-0.0),  0.0);
    }
}
