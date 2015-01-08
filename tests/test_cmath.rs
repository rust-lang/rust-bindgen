#[allow(dead_code, non_camel_case_types, non_upper_case_globals, raw_pointer_derive)]
pub mod ffi { bindgen!("/usr/include/math.h", link = "m"); }

#[test]
fn floor_is_bound_and_callable() {
    unsafe {
        assert_eq!(ffi::floor( 2.7),  2.0);
        assert_eq!(ffi::floor(-2.7), -3.0);
        assert_eq!(ffi::floor(-0.0),  0.0);
    }
}
