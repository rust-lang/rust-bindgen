#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// Stores a pointer to the ops struct, and the offset: the place to
/// write the parsed result in the destination structure.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cmdline_token_hdr {
    pub ops: *mut cmdline_token_ops,
    pub offset: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_cmdline_token_hdr() {
    const UNINIT: ::std::mem::MaybeUninit<cmdline_token_hdr> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<cmdline_token_hdr>(),
        16usize,
        concat!("Size of: ", stringify!(cmdline_token_hdr))
    );
    assert_eq!(
        ::std::mem::align_of::<cmdline_token_hdr>(),
        8usize,
        concat!("Alignment of ", stringify!(cmdline_token_hdr))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ops) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cmdline_token_hdr),
            "::",
            stringify!(ops)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offset) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cmdline_token_hdr),
            "::",
            stringify!(offset)
        )
    );
}
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
#[test]
fn bindgen_test_layout_cmdline_token_ops() {
    const UNINIT: ::std::mem::MaybeUninit<cmdline_token_ops> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<cmdline_token_ops>(),
        32usize,
        concat!("Size of: ", stringify!(cmdline_token_ops))
    );
    assert_eq!(
        ::std::mem::align_of::<cmdline_token_ops>(),
        8usize,
        concat!("Alignment of ", stringify!(cmdline_token_ops))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parse) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cmdline_token_ops),
            "::",
            stringify!(parse)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).complete_get_nb) as usize - ptr as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cmdline_token_ops),
            "::",
            stringify!(complete_get_nb)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).complete_get_elt) as usize -
                ptr as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cmdline_token_ops),
            "::",
            stringify!(complete_get_elt)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).get_help) as usize - ptr as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cmdline_token_ops),
            "::",
            stringify!(get_help)
        )
    );
}
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
#[test]
fn bindgen_test_layout_cmdline_token_num_data() {
    const UNINIT: ::std::mem::MaybeUninit<cmdline_token_num_data> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<cmdline_token_num_data>(),
        4usize,
        concat!("Size of: ", stringify!(cmdline_token_num_data))
    );
    assert_eq!(
        ::std::mem::align_of::<cmdline_token_num_data>(),
        4usize,
        concat!("Alignment of ", stringify!(cmdline_token_num_data))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cmdline_token_num_data),
            "::",
            stringify!(type_)
        )
    );
}
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
#[test]
fn bindgen_test_layout_cmdline_token_num() {
    const UNINIT: ::std::mem::MaybeUninit<cmdline_token_num> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<cmdline_token_num>(),
        24usize,
        concat!("Size of: ", stringify!(cmdline_token_num))
    );
    assert_eq!(
        ::std::mem::align_of::<cmdline_token_num>(),
        8usize,
        concat!("Alignment of ", stringify!(cmdline_token_num))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hdr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cmdline_token_num),
            "::",
            stringify!(hdr)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).num_data) as usize - ptr as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cmdline_token_num),
            "::",
            stringify!(num_data)
        )
    );
}
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
