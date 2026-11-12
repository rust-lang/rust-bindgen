#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(transparent)]
pub struct bindgen_cchar16_t(u16);
unsafe extern "C" {
    #[link_name = "_Z16receive_char16_tDs"]
    pub fn receive_char16_t(input: bindgen_cchar16_t);
}
