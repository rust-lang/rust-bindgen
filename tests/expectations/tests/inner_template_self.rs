#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LinkedList {
    pub next: *mut LinkedList,
    pub prev: *mut LinkedList,
}
impl Default for LinkedList {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InstantiateIt {
    pub m_list: LinkedList,
}
#[test]
fn bindgen_test_layout_InstantiateIt() {
    assert_eq!(
        ::std::mem::size_of::<InstantiateIt>(),
        16usize,
        concat!("Size of: ", stringify!(InstantiateIt))
    );
    assert_eq!(
        ::std::mem::align_of::<InstantiateIt>(),
        8usize,
        concat!("Alignment of ", stringify!(InstantiateIt))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<InstantiateIt>() };
            let struct_ptr = &struct_instance as *const InstantiateIt;
            let field_ptr = std::ptr::addr_of!(struct_instance.m_list);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InstantiateIt),
            "::",
            stringify!(m_list)
        )
    );
}
impl Default for InstantiateIt {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn __bindgen_test_layout_LinkedList_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<LinkedList>(),
        16usize,
        concat!("Size of template specialization: ", stringify!(LinkedList))
    );
    assert_eq!(
        ::std::mem::align_of::<LinkedList>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(LinkedList)
        )
    );
}
