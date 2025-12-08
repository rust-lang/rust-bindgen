#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct C {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 40usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 4usize];
    ["Offset of field: C::a"][::std::mem::offset_of!(C, a) - 0usize];
    ["Offset of field: C::big_array"][::std::mem::offset_of!(C, big_array) - 4usize];
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
#[repr(C)]
#[derive(Debug)]
pub struct C_with_zero_length_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of C_with_zero_length_array",
    ][::std::mem::size_of::<C_with_zero_length_array>() - 40usize];
    [
        "Alignment of C_with_zero_length_array",
    ][::std::mem::align_of::<C_with_zero_length_array>() - 4usize];
    [
        "Offset of field: C_with_zero_length_array::a",
    ][::std::mem::offset_of!(C_with_zero_length_array, a) - 0usize];
    [
        "Offset of field: C_with_zero_length_array::big_array",
    ][::std::mem::offset_of!(C_with_zero_length_array, big_array) - 4usize];
    [
        "Offset of field: C_with_zero_length_array::zero_length_array",
    ][::std::mem::offset_of!(C_with_zero_length_array, zero_length_array) - 37usize];
};
impl Default for C_with_zero_length_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_zero_length_array_2 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of C_with_zero_length_array_2",
    ][::std::mem::size_of::<C_with_zero_length_array_2>() - 4usize];
    [
        "Alignment of C_with_zero_length_array_2",
    ][::std::mem::align_of::<C_with_zero_length_array_2>() - 4usize];
    [
        "Offset of field: C_with_zero_length_array_2::a",
    ][::std::mem::offset_of!(C_with_zero_length_array_2, a) - 0usize];
    [
        "Offset of field: C_with_zero_length_array_2::zero_length_array",
    ][::std::mem::offset_of!(C_with_zero_length_array_2, zero_length_array) - 4usize];
};
#[repr(C)]
#[derive(Debug)]
pub struct C_with_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of C_with_incomplete_array",
    ][::std::mem::size_of::<C_with_incomplete_array>() - 40usize];
    [
        "Alignment of C_with_incomplete_array",
    ][::std::mem::align_of::<C_with_incomplete_array>() - 4usize];
    [
        "Offset of field: C_with_incomplete_array::a",
    ][::std::mem::offset_of!(C_with_incomplete_array, a) - 0usize];
    [
        "Offset of field: C_with_incomplete_array::big_array",
    ][::std::mem::offset_of!(C_with_incomplete_array, big_array) - 4usize];
    [
        "Offset of field: C_with_incomplete_array::incomplete_array",
    ][::std::mem::offset_of!(C_with_incomplete_array, incomplete_array) - 37usize];
};
impl Default for C_with_incomplete_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_incomplete_array_2 {
    pub a: ::std::os::raw::c_int,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of C_with_incomplete_array_2",
    ][::std::mem::size_of::<C_with_incomplete_array_2>() - 4usize];
    [
        "Alignment of C_with_incomplete_array_2",
    ][::std::mem::align_of::<C_with_incomplete_array_2>() - 4usize];
    [
        "Offset of field: C_with_incomplete_array_2::a",
    ][::std::mem::offset_of!(C_with_incomplete_array_2, a) - 0usize];
    [
        "Offset of field: C_with_incomplete_array_2::incomplete_array",
    ][::std::mem::offset_of!(C_with_incomplete_array_2, incomplete_array) - 4usize];
};
#[repr(C)]
#[derive(Debug)]
pub struct C_with_zero_length_array_and_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of C_with_zero_length_array_and_incomplete_array",
    ][::std::mem::size_of::<C_with_zero_length_array_and_incomplete_array>() - 40usize];
    [
        "Alignment of C_with_zero_length_array_and_incomplete_array",
    ][::std::mem::align_of::<C_with_zero_length_array_and_incomplete_array>() - 4usize];
    [
        "Offset of field: C_with_zero_length_array_and_incomplete_array::a",
    ][::std::mem::offset_of!(C_with_zero_length_array_and_incomplete_array, a) - 0usize];
    [
        "Offset of field: C_with_zero_length_array_and_incomplete_array::big_array",
    ][::std::mem::offset_of!(C_with_zero_length_array_and_incomplete_array, big_array)
        - 4usize];
    [
        "Offset of field: C_with_zero_length_array_and_incomplete_array::zero_length_array",
    ][::std::mem::offset_of!(
        C_with_zero_length_array_and_incomplete_array, zero_length_array
    ) - 37usize];
    [
        "Offset of field: C_with_zero_length_array_and_incomplete_array::incomplete_array",
    ][::std::mem::offset_of!(
        C_with_zero_length_array_and_incomplete_array, incomplete_array
    ) - 37usize];
};
impl Default for C_with_zero_length_array_and_incomplete_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_zero_length_array_and_incomplete_array_2 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of C_with_zero_length_array_and_incomplete_array_2",
    ][::std::mem::size_of::<C_with_zero_length_array_and_incomplete_array_2>() - 4usize];
    [
        "Alignment of C_with_zero_length_array_and_incomplete_array_2",
    ][::std::mem::align_of::<C_with_zero_length_array_and_incomplete_array_2>()
        - 4usize];
    [
        "Offset of field: C_with_zero_length_array_and_incomplete_array_2::a",
    ][::std::mem::offset_of!(C_with_zero_length_array_and_incomplete_array_2, a)
        - 0usize];
    [
        "Offset of field: C_with_zero_length_array_and_incomplete_array_2::zero_length_array",
    ][::std::mem::offset_of!(
        C_with_zero_length_array_and_incomplete_array_2, zero_length_array
    ) - 4usize];
    [
        "Offset of field: C_with_zero_length_array_and_incomplete_array_2::incomplete_array",
    ][::std::mem::offset_of!(
        C_with_zero_length_array_and_incomplete_array_2, incomplete_array
    ) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WithDtor {
    pub b: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithDtor"][::std::mem::size_of::<WithDtor>() - 4usize];
    ["Alignment of WithDtor"][::std::mem::align_of::<WithDtor>() - 4usize];
    ["Offset of field: WithDtor::b"][::std::mem::offset_of!(WithDtor, b) - 0usize];
};
#[repr(C)]
#[derive(Debug)]
pub struct IncompleteArrayNonCopiable {
    pub whatever: *mut ::std::os::raw::c_void,
    pub incomplete_array: __IncompleteArrayField<C>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of IncompleteArrayNonCopiable",
    ][::std::mem::size_of::<IncompleteArrayNonCopiable>() - 8usize];
    [
        "Alignment of IncompleteArrayNonCopiable",
    ][::std::mem::align_of::<IncompleteArrayNonCopiable>() - 8usize];
    [
        "Offset of field: IncompleteArrayNonCopiable::whatever",
    ][::std::mem::offset_of!(IncompleteArrayNonCopiable, whatever) - 0usize];
    [
        "Offset of field: IncompleteArrayNonCopiable::incomplete_array",
    ][::std::mem::offset_of!(IncompleteArrayNonCopiable, incomplete_array) - 8usize];
};
impl Default for IncompleteArrayNonCopiable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union Union {
    pub d: f32,
    pub i: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Union"][::std::mem::size_of::<Union>() - 4usize];
    ["Alignment of Union"][::std::mem::align_of::<Union>() - 4usize];
    ["Offset of field: Union::d"][::std::mem::offset_of!(Union, d) - 0usize];
    ["Offset of field: Union::i"][::std::mem::offset_of!(Union, i) - 0usize];
};
impl Default for Union {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WithUnion {
    pub data: Union,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithUnion"][::std::mem::size_of::<WithUnion>() - 4usize];
    ["Alignment of WithUnion"][::std::mem::align_of::<WithUnion>() - 4usize];
    [
        "Offset of field: WithUnion::data",
    ][::std::mem::offset_of!(WithUnion, data) - 0usize];
};
impl Default for WithUnion {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RealAbstractionWithTonsOfMethods {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RealAbstractionWithTonsOfMethods",
    ][::std::mem::size_of::<RealAbstractionWithTonsOfMethods>() - 1usize];
    [
        "Alignment of RealAbstractionWithTonsOfMethods",
    ][::std::mem::align_of::<RealAbstractionWithTonsOfMethods>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZNK32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar(
        this: *const RealAbstractionWithTonsOfMethods,
    );
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar1(
        this: *mut RealAbstractionWithTonsOfMethods,
    );
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3barEi"]
    pub fn RealAbstractionWithTonsOfMethods_bar2(
        this: *mut RealAbstractionWithTonsOfMethods,
        foo: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3staEv"]
    pub fn RealAbstractionWithTonsOfMethods_sta();
}
impl RealAbstractionWithTonsOfMethods {
    #[inline]
    pub unsafe fn bar(&self) {
        RealAbstractionWithTonsOfMethods_bar(self)
    }
    #[inline]
    pub unsafe fn bar1(&mut self) {
        RealAbstractionWithTonsOfMethods_bar1(self)
    }
    #[inline]
    pub unsafe fn bar2(&mut self, foo: ::std::os::raw::c_int) {
        RealAbstractionWithTonsOfMethods_bar2(self, foo)
    }
    #[inline]
    pub unsafe fn sta() {
        RealAbstractionWithTonsOfMethods_sta()
    }
}
