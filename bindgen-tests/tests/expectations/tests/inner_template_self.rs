#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
const _: () = {
    ["Size of InstantiateIt"][::std::mem::size_of::<InstantiateIt>() - 16usize];
    ["Alignment of InstantiateIt"][::std::mem::align_of::<InstantiateIt>() - 8usize];
    [
        "Offset of field: InstantiateIt::m_list",
    ][::std::mem::offset_of!(InstantiateIt, m_list) - 0usize];
};
impl Default for InstantiateIt {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    [
        "Size of template specialization: LinkedList_open0_int_close0",
    ][::std::mem::size_of::<LinkedList>() - 16usize];
    [
        "Align of template specialization: LinkedList_open0_int_close0",
    ][::std::mem::align_of::<LinkedList>() - 8usize];
};
