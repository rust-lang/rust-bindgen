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

#[test]
fn test_bitfields_fourth() {
    let mut fourth: bindings::bitfields::Fourth = unsafe {
        mem::zeroed()
    };
    assert!(unsafe {
        fourth.assert(bindings::bitfields::MyEnum::ONE, 0)
    });

    fourth.set_tag(bindings::bitfields::MyEnum::THREE);
    fourth.set_ptr(0xdeadbeef);
    assert!(unsafe {
        fourth.assert(bindings::bitfields::MyEnum::THREE, 0xdeadbeef)
    });
}

#[test]
fn test_bitfields_date2() {
    let mut date: bindings::bitfields::Date2 = unsafe {
        mem::zeroed()
    };
    assert!(unsafe {
        date.assert(0, 0, 0, 0, 0)
    });

    date.set_nWeekDay(6); // saturdays are the best
    date.set_nMonthDay(20);
    date.set_nMonth(11);
    date.set_nYear(95);
    date.set_byte(255);
    assert!(unsafe {
        date.assert(6, 20, 11, 95, 255)
    });
}

#[test]
fn test_bitfield_constructors() {
    use std::mem;
    let mut first = bindings::bitfields::First {
        _bitfield_1: unsafe { mem::transmute(bindings::bitfields::First::new_bitfield_1(1, 2, 3)) },
        __bindgen_align: [],
    };
    assert!(unsafe {
        first.assert(1, 2, 3)
    });

    let mut second = bindings::bitfields::Second {
        _bitfield_1: bindings::bitfields::Second::new_bitfield_1(1337, true),
        __bindgen_align: [],
    };
    assert!(unsafe {
        second.assert(1337, true)
    });

    let mut third = bindings::bitfields::Third {
        _bitfield_1: bindings::bitfields::Third::new_bitfield_1(
            42,
            false,
            bindings::bitfields::ItemKind::ITEM_KIND_TRES
        ),
        __bindgen_align: [],
    };
    assert!(unsafe {
        third.assert(42,
                     false,
                     bindings::bitfields::ItemKind::ITEM_KIND_TRES)
    });
}

impl Drop for bindings::AutoRestoreBool {
    fn drop(&mut self) {
        unsafe { bindings::AutoRestoreBool::destruct(self) }
    }
}

#[test]
fn test_destructors() {
    let mut v = true;

    {
        let auto_restore = unsafe { bindings::AutoRestoreBool::new(&mut v) };
        v = false;
    }

    assert!(v, "Should've been restored when going out of scope");
}
