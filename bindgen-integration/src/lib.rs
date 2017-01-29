mod bindings {
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

use std::ffi::CStr;
use std::os::raw::c_int;

#[test]
fn test_static_array() {
    let mut test = unsafe { bindings::Test_COUNTDOWN.as_ptr() };
    let expected = unsafe { bindings::Test_countdown()};
    let also_expected = unsafe { bindings::Test_COUNTDOWN_PTR };
    assert!(!test.is_null());
    assert_eq!(also_expected, expected);
    assert_eq!(test, also_expected);

    let mut expected = 10;
    unsafe {
        loop {
            assert_eq!(*test, expected);
            if *test == 0 {
                break;
            }
            test = test.offset(1);
            expected -= 1;
        }
    }
}

#[test]
fn test_static_method() {
    let c_str = unsafe { bindings::Test::name() };
    let name = unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() };
    assert_eq!(name, "Test", "Calling a static C++ method works!");
}

#[test]
fn test_constructor() {
    let test = unsafe { bindings::Test::new(5) };
    assert_eq!(test.m_int, 5);
    assert_eq!(test.m_double, 0.0);
}

#[test]
fn test_overload() {
    let test = unsafe { bindings::Test::new1(5.0) };
    assert_eq!(test.m_int, 0);
    assert_eq!(test.m_double, 5.0);
}
