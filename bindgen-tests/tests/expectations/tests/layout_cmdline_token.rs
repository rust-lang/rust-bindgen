#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// Stores a pointer to the ops struct, and the offset: the place to
/// write the parsed result in the destination structure.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cmdline_token_hdr {
    pub ops: *mut cmdline_token_ops,
    pub offset: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cmdline_token_hdr"][::std::mem::size_of::<cmdline_token_hdr>() - 16usize];
    [
        "Alignment of cmdline_token_hdr",
    ][::std::mem::align_of::<cmdline_token_hdr>() - 8usize];
    [
        "Offset of field: cmdline_token_hdr::ops",
    ][::std::mem::offset_of!(cmdline_token_hdr, ops) - 0usize];
    [
        "Offset of field: cmdline_token_hdr::offset",
    ][::std::mem::offset_of!(cmdline_token_hdr, offset) - 8usize];
};
impl Default for cmdline_token_hdr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// Stores a pointer to the ops struct, and the offset: the place to
/// write the parsed result in the destination structure.
pub type cmdline_parse_token_hdr_t = cmdline_token_hdr;
/// A token is defined by this structure.
///
/// parse() takes the token as first argument, then the source buffer
/// starting at the token we want to parse. The 3rd arg is a pointer
/// where we store the parsed data (as binary). It returns the number of
/// parsed chars on success and a negative value on error.
///
/// complete_get_nb() returns the number of possible values for this
/// token if completion is possible. If it is NULL or if it returns 0,
/// no completion is possible.
///
/// complete_get_elt() copy in dstbuf (the size is specified in the
/// parameter) the i-th possible completion for this token.  returns 0
/// on success or and a negative value on error.
///
/// get_help() fills the dstbuf with the help for the token. It returns
/// -1 on error and 0 on success.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct cmdline_token_ops {
    /// parse(token ptr, buf, res pts, buf len)
    pub parse: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut cmdline_parse_token_hdr_t,
            arg2: *const ::std::os::raw::c_char,
            arg3: *mut ::std::os::raw::c_void,
            arg4: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int,
    >,
    /// return the num of possible choices for this token
    pub complete_get_nb: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut cmdline_parse_token_hdr_t,
        ) -> ::std::os::raw::c_int,
    >,
    /// return the elt x for this token (token, idx, dstbuf, size)
    pub complete_get_elt: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut cmdline_parse_token_hdr_t,
            arg2: ::std::os::raw::c_int,
            arg3: *mut ::std::os::raw::c_char,
            arg4: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int,
    >,
    /// get help for this token (token, dstbuf, size)
    pub get_help: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut cmdline_parse_token_hdr_t,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cmdline_token_ops"][::std::mem::size_of::<cmdline_token_ops>() - 32usize];
    [
        "Alignment of cmdline_token_ops",
    ][::std::mem::align_of::<cmdline_token_ops>() - 8usize];
    [
        "Offset of field: cmdline_token_ops::parse",
    ][::std::mem::offset_of!(cmdline_token_ops, parse) - 0usize];
    [
        "Offset of field: cmdline_token_ops::complete_get_nb",
    ][::std::mem::offset_of!(cmdline_token_ops, complete_get_nb) - 8usize];
    [
        "Offset of field: cmdline_token_ops::complete_get_elt",
    ][::std::mem::offset_of!(cmdline_token_ops, complete_get_elt) - 16usize];
    [
        "Offset of field: cmdline_token_ops::get_help",
    ][::std::mem::offset_of!(cmdline_token_ops, get_help) - 24usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cmdline_numtype {
    UINT8 = 0,
    UINT16 = 1,
    UINT32 = 2,
    UINT64 = 3,
    INT8 = 4,
    INT16 = 5,
    INT32 = 6,
    INT64 = 7,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cmdline_token_num_data {
    pub type_: cmdline_numtype,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of cmdline_token_num_data",
    ][::std::mem::size_of::<cmdline_token_num_data>() - 4usize];
    [
        "Alignment of cmdline_token_num_data",
    ][::std::mem::align_of::<cmdline_token_num_data>() - 4usize];
    [
        "Offset of field: cmdline_token_num_data::type_",
    ][::std::mem::offset_of!(cmdline_token_num_data, type_) - 0usize];
};
impl Default for cmdline_token_num_data {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cmdline_token_num {
    pub hdr: cmdline_token_hdr,
    pub num_data: cmdline_token_num_data,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cmdline_token_num"][::std::mem::size_of::<cmdline_token_num>() - 24usize];
    [
        "Alignment of cmdline_token_num",
    ][::std::mem::align_of::<cmdline_token_num>() - 8usize];
    [
        "Offset of field: cmdline_token_num::hdr",
    ][::std::mem::offset_of!(cmdline_token_num, hdr) - 0usize];
    [
        "Offset of field: cmdline_token_num::num_data",
    ][::std::mem::offset_of!(cmdline_token_num, num_data) - 16usize];
};
impl Default for cmdline_token_num {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type cmdline_parse_token_num_t = cmdline_token_num;
