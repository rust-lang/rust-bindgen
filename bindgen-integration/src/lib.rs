#![allow(warnings)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

use std::ffi::CStr;
use std::os::raw::c_int;
use std::mem;

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

#[test]
fn test_bitfields_first() {
    let mut first: bindings::bitfields::First = unsafe {
        mem::zeroed()
    };
    assert!(unsafe {
        first.assert(0, 0, 0)
    });
    first.set_three_bits_byte_one(2);
    first.set_six_bits_byte_two(42);
    first.set_two_bits_byte_two(1);
    assert!(unsafe {
        first.assert(2, 42, 1)
    });
}

#[test]
fn test_bitfields_second() {
    let mut second: bindings::bitfields::Second = unsafe {
        mem::zeroed()
    };
    assert!(unsafe {
        second.assert(0, false)
    });
    second.set_thirty_one_bits(1337);
    second.set_one_bit(true);
    assert!(unsafe {
        second.assert(1337, true)
    });
}

#[test]
fn test_bitfields_third() {
    let mut third: bindings::bitfields::Third = unsafe {
        mem::zeroed()
    };
    assert!(unsafe {
        third.assert(0,
                     false,
                     bindings::bitfields::ItemKind::ITEM_KIND_UNO)
    });
    third.set_flags(12345);
    third.set_is_whatever(true);
    third.set_kind(bindings::bitfields::ItemKind::ITEM_KIND_TRES);
    assert!(unsafe {
        third.assert(12345,
                     true,
                     bindings::bitfields::ItemKind::ITEM_KIND_TRES)
    });
}
