#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BrowsingContext {
    pub _address: u8,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<BrowsingContext>() == 1usize,
        "Size of BrowsingContext",
    );
    assert!(
        ::std::mem::align_of::<BrowsingContext>() == 1usize,
        "Alignment of BrowsingContext",
    );
};
