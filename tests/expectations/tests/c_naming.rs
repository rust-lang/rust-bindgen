#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct struct_a {
    pub a: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_struct_a() {
    assert_eq!(
        ::std::mem::size_of::<struct_a>(),
        4usize,
        concat!("Size of: ", stringify!(struct_a))
    );
    assert_eq!(
        ::std::mem::align_of::<struct_a>(),
        4usize,
        concat!("Alignment of ", stringify!(struct_a))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<struct_a>() };
            let struct_ptr = &struct_instance as *const struct_a;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(struct_a),
            "::",
            stringify!(a)
        )
    );
}
pub type a = *const struct_a;
#[repr(C)]
#[derive(Copy, Clone)]
pub union union_b {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_union_b() {
    assert_eq!(
        ::std::mem::size_of::<union_b>(),
        4usize,
        concat!("Size of: ", stringify!(union_b))
    );
    assert_eq!(
        ::std::mem::align_of::<union_b>(),
        4usize,
        concat!("Alignment of ", stringify!(union_b))
    );
}
impl Default for union_b {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type b = union_b;
pub const enum_c_A: enum_c = 0;
pub type enum_c = ::std::os::raw::c_uint;
extern "C" {
    pub fn takes_a(arg: a);
}
extern "C" {
    pub fn takes_b(arg: b);
}
extern "C" {
    pub fn takes_c(arg: enum_c);
}
