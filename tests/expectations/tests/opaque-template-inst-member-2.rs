#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// This is like `opaque-template-inst-member.hpp` except exercising the cases
/// where we are OK to derive Debug/Hash/PartialEq.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OpaqueTemplate {
    pub _address: u8,
}
/// Should derive Debug/Hash/PartialEq.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContainsOpaqueTemplate {
    pub mBlah: u32,
    pub mBaz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ContainsOpaqueTemplate() {
    assert_eq!(
        ::std::mem::size_of::<ContainsOpaqueTemplate>(),
        8usize,
        concat!("Size of: ", stringify!(ContainsOpaqueTemplate))
    );
    assert_eq!(
        ::std::mem::align_of::<ContainsOpaqueTemplate>(),
        4usize,
        concat!("Alignment of ", stringify!(ContainsOpaqueTemplate))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<ContainsOpaqueTemplate>() };
            let struct_ptr = &struct_instance as *const ContainsOpaqueTemplate;
            let field_ptr = std::ptr::addr_of!(struct_instance.mBlah);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsOpaqueTemplate),
            "::",
            stringify!(mBlah)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<ContainsOpaqueTemplate>() };
            let struct_ptr = &struct_instance as *const ContainsOpaqueTemplate;
            let field_ptr = std::ptr::addr_of!(struct_instance.mBaz);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsOpaqueTemplate),
            "::",
            stringify!(mBaz)
        )
    );
}
/// Should also derive Debug/Hash/PartialEq.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct InheritsOpaqueTemplate {
    pub _base: u8,
    pub wow: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_InheritsOpaqueTemplate() {
    assert_eq!(
        ::std::mem::size_of::<InheritsOpaqueTemplate>(),
        16usize,
        concat!("Size of: ", stringify!(InheritsOpaqueTemplate))
    );
    assert_eq!(
        ::std::mem::align_of::<InheritsOpaqueTemplate>(),
        8usize,
        concat!("Alignment of ", stringify!(InheritsOpaqueTemplate))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<InheritsOpaqueTemplate>() };
            let struct_ptr = &struct_instance as *const InheritsOpaqueTemplate;
            let field_ptr = std::ptr::addr_of!(struct_instance.wow);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(InheritsOpaqueTemplate),
            "::",
            stringify!(wow)
        )
    );
}
impl Default for InheritsOpaqueTemplate {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
