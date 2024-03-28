#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct VirtualMethods__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VirtualMethods {
    pub vtable_: *const VirtualMethods__bindgen_vtable,
}
const _: () = {
    assert!(::std::mem::size_of::<VirtualMethods>() == 8usize, "Size of VirtualMethods");
    assert!(
        ::std::mem::align_of::<VirtualMethods>() == 8usize,
        "Alignment of VirtualMethods",
    );
};
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
const _: () = {
    assert!(
        ::std::mem::size_of::<ServoElementSnapshotTable>() == 4usize,
        "Size of ServoElementSnapshotTable",
    );
    assert!(
        ::std::mem::align_of::<ServoElementSnapshotTable>() == 4usize,
        "Alignment of ServoElementSnapshotTable",
    );
};
impl Default for ServoElementSnapshotTable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    assert!(
        ::std::mem::size_of::<Set>() == 4usize,
        "Size of template specialization: Set_open0_VirtualMethods_close0",
    );
    assert!(
        ::std::mem::align_of::<Set>() == 4usize,
        "Align of template specialization: Set_open0_VirtualMethods_close0",
    );
};
