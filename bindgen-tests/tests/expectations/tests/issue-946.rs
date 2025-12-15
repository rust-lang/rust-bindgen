#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct foo {}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 0usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 1usize];
};
pub type bar = foo;
