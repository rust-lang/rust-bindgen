#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of perf_event_attr__bindgen_ty_1",
    ][::std::mem::size_of::<perf_event_attr__bindgen_ty_1>() - 4usize];
    [
        "Alignment of perf_event_attr__bindgen_ty_1",
    ][::std::mem::align_of::<perf_event_attr__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: perf_event_attr__bindgen_ty_1::b",
    ][::std::mem::offset_of!(perf_event_attr__bindgen_ty_1, b) - 0usize];
    [
        "Offset of field: perf_event_attr__bindgen_ty_1::c",
    ][::std::mem::offset_of!(perf_event_attr__bindgen_ty_1, c) - 0usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of perf_event_attr"][::std::mem::size_of::<perf_event_attr>() - 12usize];
    ["Alignment of perf_event_attr"][::std::mem::align_of::<perf_event_attr>() - 4usize];
    [
        "Offset of field: perf_event_attr::type_",
    ][::std::mem::offset_of!(perf_event_attr, type_) - 0usize];
    [
        "Offset of field: perf_event_attr::a",
    ][::std::mem::offset_of!(perf_event_attr, a) - 4usize];
};
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
            self.type_,
            self.a,
            self.__bindgen_anon_1,
        )
    }
}
