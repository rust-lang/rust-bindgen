#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(raw_pointer_deriving)]
pub mod ffi { bindgen!("/usr/include/math.h", link = "m"); }

#[test]
fn floor_is_bound_and_callable() {
    unsafe {
        assert_eq!(ffi::floor( 2.7),  2.0);
        assert_eq!(ffi::floor(-2.7), -3.0);
        assert_eq!(ffi::floor(-0.0),  0.0);
    }
}
