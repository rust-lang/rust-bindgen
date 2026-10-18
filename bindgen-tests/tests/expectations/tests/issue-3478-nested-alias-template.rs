#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Wrapper<T> = *mut T;
pub type Id<T> = T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Holder<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub value: T,
}
impl<T> Default for Holder<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type HolderOf<T> = Holder<T>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Nested {
    pub a: Id<Id<::std::os::raw::c_int>>,
    pub b: Wrapper<Wrapper<::std::os::raw::c_char>>,
    pub c: Id<Wrapper<Id<::std::os::raw::c_int>>>,
    pub d: Holder<Holder<::std::os::raw::c_int>>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Nested"][::std::mem::size_of::<Nested>() - 32usize];
    ["Alignment of Nested"][::std::mem::align_of::<Nested>() - 8usize];
    ["Offset of field: Nested::a"][::std::mem::offset_of!(Nested, a) - 0usize];
    ["Offset of field: Nested::b"][::std::mem::offset_of!(Nested, b) - 8usize];
    ["Offset of field: Nested::c"][::std::mem::offset_of!(Nested, c) - 16usize];
    ["Offset of field: Nested::d"][::std::mem::offset_of!(Nested, d) - 24usize];
};
impl Default for Nested {
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
pub struct Generic<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub x: Id<Id<T>>,
}
impl<T> Default for Generic<T> {
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
pub struct Outer {
    pub m: Outer_Member<Outer_Member<::std::os::raw::c_int>>,
}
pub type Outer_Member<T> = T;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer"][::std::mem::size_of::<Outer>() - 4usize];
    ["Alignment of Outer"][::std::mem::align_of::<Outer>() - 4usize];
    ["Offset of field: Outer::m"][::std::mem::offset_of!(Outer, m) - 0usize];
};
impl Default for Outer {
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
pub struct Anonymous {
    pub chars: Wrapper<Wrapper<Wrapper<::std::os::raw::c_char>>>,
    pub size: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Anonymous"][::std::mem::size_of::<Anonymous>() - 16usize];
    ["Alignment of Anonymous"][::std::mem::align_of::<Anonymous>() - 8usize];
    [
        "Offset of field: Anonymous::chars",
    ][::std::mem::offset_of!(Anonymous, chars) - 0usize];
    [
        "Offset of field: Anonymous::size",
    ][::std::mem::offset_of!(Anonymous, size) - 8usize];
};
impl Default for Anonymous {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Id_open0_Id_open1_int_close1_close0",
    ][::std::mem::size_of::<Id<Id<::std::os::raw::c_int>>>() - 4usize];
    [
        "Align of template specialization: Id_open0_Id_open1_int_close1_close0",
    ][::std::mem::align_of::<Id<Id<::std::os::raw::c_int>>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Id_open0_int_close0",
    ][::std::mem::size_of::<Id<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: Id_open0_int_close0",
    ][::std::mem::align_of::<Id<::std::os::raw::c_int>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Wrapper_open0_Wrapper_open1_char_close1_close0",
    ][::std::mem::size_of::<Wrapper<Wrapper<::std::os::raw::c_char>>>() - 8usize];
    [
        "Align of template specialization: Wrapper_open0_Wrapper_open1_char_close1_close0",
    ][::std::mem::align_of::<Wrapper<Wrapper<::std::os::raw::c_char>>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Wrapper_open0_char_close0",
    ][::std::mem::size_of::<Wrapper<::std::os::raw::c_char>>() - 8usize];
    [
        "Align of template specialization: Wrapper_open0_char_close0",
    ][::std::mem::align_of::<Wrapper<::std::os::raw::c_char>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Id_open0_Wrapper_open1_Id_open2_int_close2_close1_close0",
    ][::std::mem::size_of::<Id<Wrapper<Id<::std::os::raw::c_int>>>>() - 8usize];
    [
        "Align of template specialization: Id_open0_Wrapper_open1_Id_open2_int_close2_close1_close0",
    ][::std::mem::align_of::<Id<Wrapper<Id<::std::os::raw::c_int>>>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Wrapper_open0_Id_open1_int_close1_close0",
    ][::std::mem::size_of::<Wrapper<Id<::std::os::raw::c_int>>>() - 8usize];
    [
        "Align of template specialization: Wrapper_open0_Id_open1_int_close1_close0",
    ][::std::mem::align_of::<Wrapper<Id<::std::os::raw::c_int>>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Id_open0_int_close0",
    ][::std::mem::size_of::<Id<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: Id_open0_int_close0",
    ][::std::mem::align_of::<Id<::std::os::raw::c_int>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Holder_open0_Holder_open1_int_close1_close0",
    ][::std::mem::size_of::<Holder<Holder<::std::os::raw::c_int>>>() - 4usize];
    [
        "Align of template specialization: Holder_open0_Holder_open1_int_close1_close0",
    ][::std::mem::align_of::<Holder<Holder<::std::os::raw::c_int>>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Holder_open0_int_close0",
    ][::std::mem::size_of::<Holder<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: Holder_open0_int_close0",
    ][::std::mem::align_of::<Holder<::std::os::raw::c_int>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Outer_Member_open0_Outer_Member_open1_int_close1_close0",
    ][::std::mem::size_of::<Outer_Member<Outer_Member<::std::os::raw::c_int>>>()
        - 4usize];
    [
        "Align of template specialization: Outer_Member_open0_Outer_Member_open1_int_close1_close0",
    ][::std::mem::align_of::<Outer_Member<Outer_Member<::std::os::raw::c_int>>>()
        - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Outer_Member_open0_int_close0",
    ][::std::mem::size_of::<Outer_Member<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: Outer_Member_open0_int_close0",
    ][::std::mem::align_of::<Outer_Member<::std::os::raw::c_int>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Wrapper_open0_Wrapper_open1_Wrapper_open2_char_close2_close1_close0",
    ][::std::mem::size_of::<Wrapper<Wrapper<Wrapper<::std::os::raw::c_char>>>>()
        - 8usize];
    [
        "Align of template specialization: Wrapper_open0_Wrapper_open1_Wrapper_open2_char_close2_close1_close0",
    ][::std::mem::align_of::<Wrapper<Wrapper<Wrapper<::std::os::raw::c_char>>>>()
        - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Wrapper_open0_Wrapper_open1_char_close1_close0",
    ][::std::mem::size_of::<Wrapper<Wrapper<::std::os::raw::c_char>>>() - 8usize];
    [
        "Align of template specialization: Wrapper_open0_Wrapper_open1_char_close1_close0",
    ][::std::mem::align_of::<Wrapper<Wrapper<::std::os::raw::c_char>>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Wrapper_open0_char_close0",
    ][::std::mem::size_of::<Wrapper<::std::os::raw::c_char>>() - 8usize];
    [
        "Align of template specialization: Wrapper_open0_char_close0",
    ][::std::mem::align_of::<Wrapper<::std::os::raw::c_char>>() - 8usize];
};
