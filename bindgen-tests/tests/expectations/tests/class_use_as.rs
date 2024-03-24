#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen="true" replaces="whatever"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever {
    pub replacement: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<whatever>() == 4usize, "Size of whatever");
    assert!(::std::mem::align_of::<whatever>() == 4usize, "Alignment of whatever");
    assert!(
        ::std::mem::offset_of!(whatever, replacement) == 0usize,
        "Offset of field: whatever::replacement",
    );
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct container {
    pub c: whatever,
}
const _: () = {
    assert!(::std::mem::size_of::<container>() == 4usize, "Size of container");
    assert!(::std::mem::align_of::<container>() == 4usize, "Alignment of container");
    assert!(
        ::std::mem::offset_of!(container, c) == 0usize,
        "Offset of field: container::c",
    );
};
