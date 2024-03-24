#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct A {
    pub c: ::std::os::raw::c_uint,
    pub named_union: A__bindgen_ty_1,
    pub __bindgen_anon_1: A__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of A_Segment"][::std::mem::size_of::<A_Segment>() - 8usize];
    ["Alignment of A_Segment"][::std::mem::align_of::<A_Segment>() - 4usize];
    [
        "Offset of field: A_Segment::begin",
    ][::std::mem::offset_of!(A_Segment, begin) - 0usize];
    ["Offset of field: A_Segment::end"][::std::mem::offset_of!(A_Segment, end) - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union A__bindgen_ty_1 {
    pub f: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of A__bindgen_ty_1"][::std::mem::size_of::<A__bindgen_ty_1>() - 4usize];
    ["Alignment of A__bindgen_ty_1"][::std::mem::align_of::<A__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: A__bindgen_ty_1::f",
    ][::std::mem::offset_of!(A__bindgen_ty_1, f) - 0usize];
};
impl Default for A__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union A__bindgen_ty_2 {
    pub d: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of A__bindgen_ty_2"][::std::mem::size_of::<A__bindgen_ty_2>() - 4usize];
    ["Alignment of A__bindgen_ty_2"][::std::mem::align_of::<A__bindgen_ty_2>() - 4usize];
    [
        "Offset of field: A__bindgen_ty_2::d",
    ][::std::mem::offset_of!(A__bindgen_ty_2, d) - 0usize];
};
impl Default for A__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 12usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 4usize];
    ["Offset of field: A::c"][::std::mem::offset_of!(A, c) - 0usize];
    ["Offset of field: A::named_union"][::std::mem::offset_of!(A, named_union) - 4usize];
};
impl Default for A {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B {
    pub d: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of B_Segment"][::std::mem::size_of::<B_Segment>() - 8usize];
    ["Alignment of B_Segment"][::std::mem::align_of::<B_Segment>() - 4usize];
    [
        "Offset of field: B_Segment::begin",
    ][::std::mem::offset_of!(B_Segment, begin) - 0usize];
    ["Offset of field: B_Segment::end"][::std::mem::offset_of!(B_Segment, end) - 4usize];
};
const _: () = {
    ["Size of B"][::std::mem::size_of::<B>() - 4usize];
    ["Alignment of B"][::std::mem::align_of::<B>() - 4usize];
    ["Offset of field: B::d"][::std::mem::offset_of!(B, d) - 0usize];
};
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StepSyntax {
    Keyword = 0,
    FunctionalWithoutKeyword = 1,
    FunctionalWithStartKeyword = 2,
    FunctionalWithEndKeyword = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct C {
    pub d: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: C__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union C__bindgen_ty_1 {
    pub mFunc: C__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_1: C__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct C__bindgen_ty_1__bindgen_ty_1 {
    pub mX1: f32,
    pub mY1: f32,
    pub mX2: f32,
    pub mY2: f32,
}
const _: () = {
    [
        "Size of C__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<C__bindgen_ty_1__bindgen_ty_1>() - 16usize];
    [
        "Alignment of C__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<C__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: C__bindgen_ty_1__bindgen_ty_1::mX1",
    ][::std::mem::offset_of!(C__bindgen_ty_1__bindgen_ty_1, mX1) - 0usize];
    [
        "Offset of field: C__bindgen_ty_1__bindgen_ty_1::mY1",
    ][::std::mem::offset_of!(C__bindgen_ty_1__bindgen_ty_1, mY1) - 4usize];
    [
        "Offset of field: C__bindgen_ty_1__bindgen_ty_1::mX2",
    ][::std::mem::offset_of!(C__bindgen_ty_1__bindgen_ty_1, mX2) - 8usize];
    [
        "Offset of field: C__bindgen_ty_1__bindgen_ty_1::mY2",
    ][::std::mem::offset_of!(C__bindgen_ty_1__bindgen_ty_1, mY2) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct C__bindgen_ty_1__bindgen_ty_2 {
    pub mStepSyntax: StepSyntax,
    pub mSteps: ::std::os::raw::c_uint,
}
const _: () = {
    [
        "Size of C__bindgen_ty_1__bindgen_ty_2",
    ][::std::mem::size_of::<C__bindgen_ty_1__bindgen_ty_2>() - 8usize];
    [
        "Alignment of C__bindgen_ty_1__bindgen_ty_2",
    ][::std::mem::align_of::<C__bindgen_ty_1__bindgen_ty_2>() - 4usize];
    [
        "Offset of field: C__bindgen_ty_1__bindgen_ty_2::mStepSyntax",
    ][::std::mem::offset_of!(C__bindgen_ty_1__bindgen_ty_2, mStepSyntax) - 0usize];
    [
        "Offset of field: C__bindgen_ty_1__bindgen_ty_2::mSteps",
    ][::std::mem::offset_of!(C__bindgen_ty_1__bindgen_ty_2, mSteps) - 4usize];
};
impl Default for C__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    ["Size of C__bindgen_ty_1"][::std::mem::size_of::<C__bindgen_ty_1>() - 16usize];
    ["Alignment of C__bindgen_ty_1"][::std::mem::align_of::<C__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: C__bindgen_ty_1::mFunc",
    ][::std::mem::offset_of!(C__bindgen_ty_1, mFunc) - 0usize];
};
impl Default for C__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct C_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of C_Segment"][::std::mem::size_of::<C_Segment>() - 8usize];
    ["Alignment of C_Segment"][::std::mem::align_of::<C_Segment>() - 4usize];
    [
        "Offset of field: C_Segment::begin",
    ][::std::mem::offset_of!(C_Segment, begin) - 0usize];
    ["Offset of field: C_Segment::end"][::std::mem::offset_of!(C_Segment, end) - 4usize];
};
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 20usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 4usize];
    ["Offset of field: C::d"][::std::mem::offset_of!(C, d) - 0usize];
};
impl Default for C {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
