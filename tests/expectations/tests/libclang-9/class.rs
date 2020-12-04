#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

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
#[derive(Copy, Clone)]
pub struct C {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        40usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        4usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).big_array as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(big_array)
        )
    );
}
impl Default for C {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_C {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_C {}
impl Drop for Box_C {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(40usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
pub struct C_with_zero_length_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array() {
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array>(),
        40usize,
        concat!("Size of: ", stringify!(C_with_zero_length_array))
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array>(),
        4usize,
        concat!("Alignment of ", stringify!(C_with_zero_length_array))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_zero_length_array>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_zero_length_array>())).big_array
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array),
            "::",
            stringify!(big_array)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_zero_length_array>()))
                .zero_length_array as *const _ as usize
        },
        37usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array),
            "::",
            stringify!(zero_length_array)
        )
    );
}
impl Default for C_with_zero_length_array {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_C_with_zero_length_array {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_C_with_zero_length_array {}
impl Drop for Box_C_with_zero_length_array {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(40usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_zero_length_array_2 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array_2() {
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array_2>(),
        4usize,
        concat!("Size of: ", stringify!(C_with_zero_length_array_2))
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array_2>(),
        4usize,
        concat!("Alignment of ", stringify!(C_with_zero_length_array_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_zero_length_array_2>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array_2),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_zero_length_array_2>()))
                .zero_length_array as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array_2),
            "::",
            stringify!(zero_length_array)
        )
    );
}
struct Box_C_with_zero_length_array_2 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_C_with_zero_length_array_2 {}
impl Drop for Box_C_with_zero_length_array_2 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
pub struct C_with_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_incomplete_array() {
    assert_eq!(
        ::std::mem::size_of::<C_with_incomplete_array>(),
        40usize,
        concat!("Size of: ", stringify!(C_with_incomplete_array))
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_incomplete_array>(),
        4usize,
        concat!("Alignment of ", stringify!(C_with_incomplete_array))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_incomplete_array>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_incomplete_array),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_incomplete_array>())).big_array
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_incomplete_array),
            "::",
            stringify!(big_array)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_incomplete_array>())).incomplete_array
                as *const _ as usize
        },
        37usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_incomplete_array),
            "::",
            stringify!(incomplete_array)
        )
    );
}
impl Default for C_with_incomplete_array {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_C_with_incomplete_array {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_C_with_incomplete_array {}
impl Drop for Box_C_with_incomplete_array {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(40usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_incomplete_array_2 {
    pub a: ::std::os::raw::c_int,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_incomplete_array_2() {
    assert_eq!(
        ::std::mem::size_of::<C_with_incomplete_array_2>(),
        4usize,
        concat!("Size of: ", stringify!(C_with_incomplete_array_2))
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_incomplete_array_2>(),
        4usize,
        concat!("Alignment of ", stringify!(C_with_incomplete_array_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_incomplete_array_2>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_incomplete_array_2),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C_with_incomplete_array_2>()))
                .incomplete_array as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_incomplete_array_2),
            "::",
            stringify!(incomplete_array)
        )
    );
}
struct Box_C_with_incomplete_array_2 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_C_with_incomplete_array_2 {}
impl Drop for Box_C_with_incomplete_array_2 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
pub struct C_with_zero_length_array_and_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array_and_incomplete_array() {
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array_and_incomplete_array>(),
        40usize,
        concat!(
            "Size of: ",
            stringify!(C_with_zero_length_array_and_incomplete_array)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array_and_incomplete_array>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(C_with_zero_length_array_and_incomplete_array)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                C_with_zero_length_array_and_incomplete_array,
            >()))
            .a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array_and_incomplete_array),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                C_with_zero_length_array_and_incomplete_array,
            >()))
            .big_array as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array_and_incomplete_array),
            "::",
            stringify!(big_array)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                C_with_zero_length_array_and_incomplete_array,
            >()))
            .zero_length_array as *const _ as usize
        },
        37usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array_and_incomplete_array),
            "::",
            stringify!(zero_length_array)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                C_with_zero_length_array_and_incomplete_array,
            >()))
            .incomplete_array as *const _ as usize
        },
        37usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array_and_incomplete_array),
            "::",
            stringify!(incomplete_array)
        )
    );
}
impl Default for C_with_zero_length_array_and_incomplete_array {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_C_with_zero_length_array_and_incomplete_array {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_C_with_zero_length_array_and_incomplete_array {}
impl Drop for Box_C_with_zero_length_array_and_incomplete_array {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(40usize, 4usize).unwrap(),
            );
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
#[test]
fn bindgen_test_layout_C_with_zero_length_array_and_incomplete_array_2() {
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array_and_incomplete_array_2>(
        ),
        4usize,
        concat!(
            "Size of: ",
            stringify!(C_with_zero_length_array_and_incomplete_array_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array_and_incomplete_array_2>(
        ),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(C_with_zero_length_array_and_incomplete_array_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                C_with_zero_length_array_and_incomplete_array_2,
            >()))
            .a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array_and_incomplete_array_2),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                C_with_zero_length_array_and_incomplete_array_2,
            >()))
            .zero_length_array as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array_and_incomplete_array_2),
            "::",
            stringify!(zero_length_array)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                C_with_zero_length_array_and_incomplete_array_2,
            >()))
            .incomplete_array as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C_with_zero_length_array_and_incomplete_array_2),
            "::",
            stringify!(incomplete_array)
        )
    );
}
struct Box_C_with_zero_length_array_and_incomplete_array_2 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_C_with_zero_length_array_and_incomplete_array_2 {}
impl Drop for Box_C_with_zero_length_array_and_incomplete_array_2 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct WithDtor {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WithDtor() {
    assert_eq!(
        ::std::mem::size_of::<WithDtor>(),
        4usize,
        concat!("Size of: ", stringify!(WithDtor))
    );
    assert_eq!(
        ::std::mem::align_of::<WithDtor>(),
        4usize,
        concat!("Alignment of ", stringify!(WithDtor))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithDtor>())).b as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithDtor),
            "::",
            stringify!(b)
        )
    );
}
struct Box_WithDtor {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_WithDtor {}
impl Drop for Box_WithDtor {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
pub struct IncompleteArrayNonCopiable {
    pub whatever: *mut ::std::os::raw::c_void,
    pub incomplete_array: __IncompleteArrayField<C>,
}
#[test]
fn bindgen_test_layout_IncompleteArrayNonCopiable() {
    assert_eq!(
        ::std::mem::size_of::<IncompleteArrayNonCopiable>(),
        8usize,
        concat!("Size of: ", stringify!(IncompleteArrayNonCopiable))
    );
    assert_eq!(
        ::std::mem::align_of::<IncompleteArrayNonCopiable>(),
        8usize,
        concat!("Alignment of ", stringify!(IncompleteArrayNonCopiable))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<IncompleteArrayNonCopiable>())).whatever
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(IncompleteArrayNonCopiable),
            "::",
            stringify!(whatever)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<IncompleteArrayNonCopiable>()))
                .incomplete_array as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(IncompleteArrayNonCopiable),
            "::",
            stringify!(incomplete_array)
        )
    );
}
impl Default for IncompleteArrayNonCopiable {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_IncompleteArrayNonCopiable {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_IncompleteArrayNonCopiable {}
impl Drop for Box_IncompleteArrayNonCopiable {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(8usize, 8usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Union {
    pub d: f32,
    pub i: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_Union() {
    assert_eq!(
        ::std::mem::size_of::<Union>(),
        4usize,
        concat!("Size of: ", stringify!(Union))
    );
    assert_eq!(
        ::std::mem::align_of::<Union>(),
        4usize,
        concat!("Alignment of ", stringify!(Union))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Union>())).d as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Union), "::", stringify!(d))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Union>())).i as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Union), "::", stringify!(i))
    );
}
impl Default for Union {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_Union {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Union {}
impl Drop for Box_Union {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WithUnion {
    pub data: Union,
}
#[test]
fn bindgen_test_layout_WithUnion() {
    assert_eq!(
        ::std::mem::size_of::<WithUnion>(),
        4usize,
        concat!("Size of: ", stringify!(WithUnion))
    );
    assert_eq!(
        ::std::mem::align_of::<WithUnion>(),
        4usize,
        concat!("Alignment of ", stringify!(WithUnion))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithUnion>())).data as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithUnion),
            "::",
            stringify!(data)
        )
    );
}
impl Default for WithUnion {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_WithUnion {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_WithUnion {}
impl Drop for Box_WithUnion {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct RealAbstractionWithTonsOfMethods {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_RealAbstractionWithTonsOfMethods() {
    assert_eq!(
        ::std::mem::size_of::<RealAbstractionWithTonsOfMethods>(),
        1usize,
        concat!("Size of: ", stringify!(RealAbstractionWithTonsOfMethods))
    );
    assert_eq!(
        ::std::mem::align_of::<RealAbstractionWithTonsOfMethods>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(RealAbstractionWithTonsOfMethods)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNK32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar(
        this: *const RealAbstractionWithTonsOfMethods,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar1(
        this: *mut RealAbstractionWithTonsOfMethods,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3barEi"]
    pub fn RealAbstractionWithTonsOfMethods_bar2(
        this: *mut RealAbstractionWithTonsOfMethods,
        foo: ::std::os::raw::c_int,
    );
}
extern "C" {
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
struct Box_RealAbstractionWithTonsOfMethods {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_RealAbstractionWithTonsOfMethods {
    #[inline]
    pub fn bar(&self) {
        unsafe {
            RealAbstractionWithTonsOfMethods_bar(
                self.ptr as *const RealAbstractionWithTonsOfMethods,
            )
        }
    }
    #[inline]
    pub fn bar1(&mut self) {
        unsafe {
            RealAbstractionWithTonsOfMethods_bar1(
                self.ptr as *mut RealAbstractionWithTonsOfMethods,
            )
        }
    }
    #[inline]
    pub fn bar2(&mut self, foo: ::std::os::raw::c_int) {
        unsafe {
            RealAbstractionWithTonsOfMethods_bar2(
                self.ptr as *mut RealAbstractionWithTonsOfMethods,
                foo,
            )
        }
    }
    #[inline]
    pub fn sta() {
        unsafe { RealAbstractionWithTonsOfMethods_sta() }
    }
}
impl Drop for Box_RealAbstractionWithTonsOfMethods {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
