#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    pub type_: ::std::os::raw::c_uint,
    pub a: f32,
    pub __bindgen_anon_1: perf_event_attr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr__bindgen_ty_1 {
    pub b: ::std::os::raw::c_int,
    pub c: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_perf_event_attr__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<perf_event_attr__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(perf_event_attr__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_attr__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(perf_event_attr__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<perf_event_attr__bindgen_ty_1>())).b
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<perf_event_attr__bindgen_ty_1>())).c
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr__bindgen_ty_1),
            "::",
            stringify!(c)
        )
    );
}
impl Default for perf_event_attr__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_attr__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "perf_event_attr__bindgen_ty_1 {{ union }}")
    }
}
#[test]
fn bindgen_test_layout_perf_event_attr() {
    assert_eq!(
        ::std::mem::size_of::<perf_event_attr>(),
        12usize,
        concat!("Size of: ", stringify!(perf_event_attr))
    );
    assert_eq!(
        ::std::mem::align_of::<perf_event_attr>(),
        4usize,
        concat!("Alignment of ", stringify!(perf_event_attr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<perf_event_attr>())).type_ as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<perf_event_attr>())).a as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(perf_event_attr),
            "::",
            stringify!(a)
        )
    );
}
impl Default for perf_event_attr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for perf_event_attr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "perf_event_attr {{ type: {:?}, a: {:?}, __bindgen_anon_1: {:?} }}",
            self.type_, self.a, self.__bindgen_anon_1
        )
    }
}
