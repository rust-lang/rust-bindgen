#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct S {
    /// <div rustbindgen attribute="#[deprecated] #[cfg(not(any(foo, bar)))]"></div>
    #[deprecated]
    #[cfg(not(any(foo, bar)))]
    pub field_1: ::std::os::raw::c_int,
    /// <div rustbindgen attribute="#[cfg(not(baz))]"></div>
    #[cfg(not(baz))]
    pub field_2: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of S"][::std::mem::size_of::<S>() - 8usize];
    ["Alignment of S"][::std::mem::align_of::<S>() - 4usize];
    ["Offset of field: S::field_1"][::std::mem::offset_of!(S, field_1) - 0usize];
    ["Offset of field: S::field_2"][::std::mem::offset_of!(S, field_2) - 4usize];
};
