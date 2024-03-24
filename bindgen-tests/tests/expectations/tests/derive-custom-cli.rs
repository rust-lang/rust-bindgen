#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Clone, Default)]
pub struct foo_struct {
    pub inner: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<foo_struct>() == 4usize, "Size of foo_struct");
    assert!(::std::mem::align_of::<foo_struct>() == 4usize, "Alignment of foo_struct");
    assert!(
        ::std::mem::offset_of!(foo_struct, inner) == 0usize,
        "Offset of field: foo_struct::inner",
    );
};
#[repr(u32)]
#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub enum foo_enum {
    inner = 0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union foo_union {
    pub fst: ::std::mem::ManuallyDrop<::std::os::raw::c_int>,
    pub snd: ::std::mem::ManuallyDrop<f32>,
}
const _: () = {
    assert!(::std::mem::size_of::<foo_union>() == 4usize, "Size of foo_union");
    assert!(::std::mem::align_of::<foo_union>() == 4usize, "Alignment of foo_union");
    assert!(
        ::std::mem::offset_of!(foo_union, fst) == 0usize,
        "Offset of field: foo_union::fst",
    );
    assert!(
        ::std::mem::offset_of!(foo_union, snd) == 0usize,
        "Offset of field: foo_union::snd",
    );
};
#[repr(C)]
pub struct non_matching {
    pub inner: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<non_matching>() == 4usize, "Size of non_matching");
    assert!(
        ::std::mem::align_of::<non_matching>() == 4usize,
        "Alignment of non_matching",
    );
    assert!(
        ::std::mem::offset_of!(non_matching, inner) == 0usize,
        "Offset of field: non_matching::inner",
    );
};
