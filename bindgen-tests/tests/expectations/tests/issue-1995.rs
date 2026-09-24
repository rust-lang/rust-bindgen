#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// This is a constant that has a docstring
///
/// And expected to be found in generated bindings code too.
pub const FOO: ::std::os::raw::c_int = 1;
/// This is a constant that has a docstring
///
/// And expected to be found in generated bindings code too.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub baz: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 4usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 4usize];
    ["Offset of field: Bar::baz"][::std::mem::offset_of!(Bar, baz) - 0usize];
};
