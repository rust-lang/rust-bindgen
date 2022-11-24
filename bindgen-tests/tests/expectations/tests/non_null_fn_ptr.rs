#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub bar: unsafe extern "C" fn() -> ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo() {
    const UNINIT: ::std::mem::MaybeUninit<foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        8usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
    );
}
extern "C" {
    pub fn new_foo(arg: unsafe extern "C" fn() -> ::std::os::raw::c_int)
        -> foo;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub foo:
        ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
}
#[test]
fn bindgen_test_layout_baz() {
    const UNINIT: ::std::mem::MaybeUninit<baz> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<baz>(),
        8usize,
        concat!("Size of: ", stringify!(baz))
    );
    assert_eq!(
        ::std::mem::align_of::<baz>(),
        8usize,
        concat!("Alignment of ", stringify!(baz))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(baz), "::", stringify!(foo))
    );
}
extern "C" {
    pub fn new_baz(
        foo: ::std::option::Option<
            unsafe extern "C" fn() -> ::std::os::raw::c_int,
        >,
    ) -> baz;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union union_foo {
    pub fst: unsafe extern "C" fn() -> ::std::os::raw::c_int,
    pub snd: unsafe extern "C" fn() -> f32,
}
#[test]
fn bindgen_test_layout_union_foo() {
    const UNINIT: ::std::mem::MaybeUninit<union_foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<union_foo>(),
        8usize,
        concat!("Size of: ", stringify!(union_foo))
    );
    assert_eq!(
        ::std::mem::align_of::<union_foo>(),
        8usize,
        concat!("Alignment of ", stringify!(union_foo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fst) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(union_foo),
            "::",
            stringify!(fst)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).snd) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(union_foo),
            "::",
            stringify!(snd)
        )
    );
}
extern "C" {
    pub fn new_union_foo(
        arg: unsafe extern "C" fn() -> ::std::os::raw::c_int,
    ) -> union_foo;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union union_baz {
    pub foo:
        ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub bar: ::std::option::Option<unsafe extern "C" fn() -> f32>,
}
#[test]
fn bindgen_test_layout_union_baz() {
    const UNINIT: ::std::mem::MaybeUninit<union_baz> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<union_baz>(),
        8usize,
        concat!("Size of: ", stringify!(union_baz))
    );
    assert_eq!(
        ::std::mem::align_of::<union_baz>(),
        8usize,
        concat!("Alignment of ", stringify!(union_baz))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(union_baz),
            "::",
            stringify!(foo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(union_baz),
            "::",
            stringify!(bar)
        )
    );
}
impl Default for union_baz {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn new_union_baz(
        foo: ::std::option::Option<
            unsafe extern "C" fn() -> ::std::os::raw::c_int,
        >,
    ) -> union_baz;
}
