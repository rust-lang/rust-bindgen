#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub enum Bar {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub baz: *mut Bar,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        8usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        8usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    fn test_field_baz() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baz) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(baz)
            )
        );
    }
    test_field_baz();
}
impl Default for Foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
