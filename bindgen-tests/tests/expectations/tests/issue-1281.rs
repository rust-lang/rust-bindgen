#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar {
    pub u: foo,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub foo: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<foo>() == 4usize, "Size of foo");
    assert!(::std::mem::align_of::<foo>() == 4usize, "Alignment of foo");
    assert!(::std::mem::offset_of!(foo, foo) == 0usize, "Offset of field: foo::foo");
};
const _: () = {
    assert!(::std::mem::size_of::<bar>() == 4usize, "Size of bar");
    assert!(::std::mem::align_of::<bar>() == 4usize, "Alignment of bar");
    assert!(::std::mem::offset_of!(bar, u) == 0usize, "Offset of field: bar::u");
};
pub type bar_t = bar;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub f: foo,
}
const _: () = {
    assert!(::std::mem::size_of::<baz>() == 4usize, "Size of baz");
    assert!(::std::mem::align_of::<baz>() == 4usize, "Alignment of baz");
    assert!(::std::mem::offset_of!(baz, f) == 0usize, "Offset of field: baz::f");
};
