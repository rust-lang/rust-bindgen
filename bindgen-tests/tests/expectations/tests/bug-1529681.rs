#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BrowsingContext {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BrowsingContext"][::std::mem::size_of::<BrowsingContext>() - 1usize];
    ["Alignment of BrowsingContext"][::std::mem::align_of::<BrowsingContext>() - 1usize];
};
