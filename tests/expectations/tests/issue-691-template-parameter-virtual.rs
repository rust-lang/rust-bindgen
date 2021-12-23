#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct VirtualMethods__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VirtualMethods {
    pub vtable_: *const VirtualMethods__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_VirtualMethods() {
    assert_eq!(
        ::std::mem::size_of::<VirtualMethods>(),
        8usize,
        concat!("Size of: ", stringify!(VirtualMethods))
    );
    assert_eq!(
        ::std::mem::align_of::<VirtualMethods>(),
        8usize,
        concat!("Alignment of ", stringify!(VirtualMethods))
    );
}
impl Default for VirtualMethods {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Set {
    pub bar: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ServoElementSnapshotTable {
    pub _base: Set,
}
#[test]
fn bindgen_test_layout_ServoElementSnapshotTable() {
    assert_eq!(
        ::std::mem::size_of::<ServoElementSnapshotTable>(),
        4usize,
        concat!("Size of: ", stringify!(ServoElementSnapshotTable))
    );
    assert_eq!(
        ::std::mem::align_of::<ServoElementSnapshotTable>(),
        4usize,
        concat!("Alignment of ", stringify!(ServoElementSnapshotTable))
    );
}
impl Default for ServoElementSnapshotTable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn __bindgen_test_layout_Set_open0_VirtualMethods_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Set>(),
        4usize,
        concat!("Size of template specialization: ", stringify!(Set))
    );
    assert_eq!(
        ::std::mem::align_of::<Set>(),
        4usize,
        concat!("Alignment of template specialization: ", stringify!(Set))
    );
}
