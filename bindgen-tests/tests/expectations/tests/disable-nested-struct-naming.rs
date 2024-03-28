#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub b1: bar1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar1 {
    pub x1: ::std::os::raw::c_int,
    pub b2: bar1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar1__bindgen_ty_1 {
    pub x2: ::std::os::raw::c_int,
    pub b3: bar1__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar1__bindgen_ty_1__bindgen_ty_1 {
    pub x3: ::std::os::raw::c_int,
    pub b4: bar4,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar4 {
    pub x4: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<bar4>() == 4usize, "Size of bar4");
    assert!(::std::mem::align_of::<bar4>() == 4usize, "Alignment of bar4");
    assert!(::std::mem::offset_of!(bar4, x4) == 0usize, "Offset of field: bar4::x4");
};
const _: () = {
    assert!(
        ::std::mem::size_of::<bar1__bindgen_ty_1__bindgen_ty_1>() == 8usize,
        "Size of bar1__bindgen_ty_1__bindgen_ty_1",
    );
    assert!(
        ::std::mem::align_of::<bar1__bindgen_ty_1__bindgen_ty_1>() == 4usize,
        "Alignment of bar1__bindgen_ty_1__bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(bar1__bindgen_ty_1__bindgen_ty_1, x3) == 0usize,
        "Offset of field: bar1__bindgen_ty_1__bindgen_ty_1::x3",
    );
    assert!(
        ::std::mem::offset_of!(bar1__bindgen_ty_1__bindgen_ty_1, b4) == 4usize,
        "Offset of field: bar1__bindgen_ty_1__bindgen_ty_1::b4",
    );
};
const _: () = {
    assert!(
        ::std::mem::size_of::<bar1__bindgen_ty_1>() == 12usize,
        "Size of bar1__bindgen_ty_1",
    );
    assert!(
        ::std::mem::align_of::<bar1__bindgen_ty_1>() == 4usize,
        "Alignment of bar1__bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(bar1__bindgen_ty_1, x2) == 0usize,
        "Offset of field: bar1__bindgen_ty_1::x2",
    );
    assert!(
        ::std::mem::offset_of!(bar1__bindgen_ty_1, b3) == 4usize,
        "Offset of field: bar1__bindgen_ty_1::b3",
    );
};
const _: () = {
    assert!(::std::mem::size_of::<bar1>() == 16usize, "Size of bar1");
    assert!(::std::mem::align_of::<bar1>() == 4usize, "Alignment of bar1");
    assert!(::std::mem::offset_of!(bar1, x1) == 0usize, "Offset of field: bar1::x1");
    assert!(::std::mem::offset_of!(bar1, b2) == 4usize, "Offset of field: bar1::b2");
};
const _: () = {
    assert!(::std::mem::size_of::<foo>() == 16usize, "Size of foo");
    assert!(::std::mem::align_of::<foo>() == 4usize, "Alignment of foo");
    assert!(::std::mem::offset_of!(foo, b1) == 0usize, "Offset of field: foo::b1");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub anon2: _bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1__bindgen_ty_1 {
    pub b: baz,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub x: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<baz>() == 4usize, "Size of baz");
    assert!(::std::mem::align_of::<baz>() == 4usize, "Alignment of baz");
    assert!(::std::mem::offset_of!(baz, x) == 0usize, "Offset of field: baz::x");
};
const _: () = {
    assert!(
        ::std::mem::size_of::<_bindgen_ty_1__bindgen_ty_1>() == 4usize,
        "Size of _bindgen_ty_1__bindgen_ty_1",
    );
    assert!(
        ::std::mem::align_of::<_bindgen_ty_1__bindgen_ty_1>() == 4usize,
        "Alignment of _bindgen_ty_1__bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(_bindgen_ty_1__bindgen_ty_1, b) == 0usize,
        "Offset of field: _bindgen_ty_1__bindgen_ty_1::b",
    );
};
const _: () = {
    assert!(::std::mem::size_of::<_bindgen_ty_1>() == 4usize, "Size of _bindgen_ty_1");
    assert!(
        ::std::mem::align_of::<_bindgen_ty_1>() == 4usize,
        "Alignment of _bindgen_ty_1",
    );
    assert!(
        ::std::mem::offset_of!(_bindgen_ty_1, anon2) == 0usize,
        "Offset of field: _bindgen_ty_1::anon2",
    );
};
extern "C" {
    pub static mut anon1: _bindgen_ty_1;
}
