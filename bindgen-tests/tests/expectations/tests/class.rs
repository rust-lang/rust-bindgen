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
#[derive(Copy, Clone)]
pub struct C {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
}
#[test]
fn bindgen_test_layout_C() {
    const UNINIT: ::std::mem::MaybeUninit<C> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<C>(), 40usize, "Size of C");
    assert_eq!(::std::mem::align_of::<C>(), 4usize, "Alignment of C");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: C::a",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).big_array) as usize - ptr as usize },
        4usize,
        "Offset of field: C::big_array",
    );
}
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
pub struct C_with_zero_length_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array() {
    const UNINIT: ::std::mem::MaybeUninit<C_with_zero_length_array> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array>(),
        40usize,
        "Size of C_with_zero_length_array",
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array>(),
        4usize,
        "Alignment of C_with_zero_length_array",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: C_with_zero_length_array::a",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).big_array) as usize - ptr as usize },
        4usize,
        "Offset of field: C_with_zero_length_array::big_array",
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).zero_length_array) as usize - ptr as usize
        },
        37usize,
        "Offset of field: C_with_zero_length_array::zero_length_array",
    );
}
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
#[test]
fn bindgen_test_layout_C_with_zero_length_array_2() {
    const UNINIT: ::std::mem::MaybeUninit<C_with_zero_length_array_2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array_2>(),
        4usize,
        "Size of C_with_zero_length_array_2",
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array_2>(),
        4usize,
        "Alignment of C_with_zero_length_array_2",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: C_with_zero_length_array_2::a",
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).zero_length_array) as usize - ptr as usize
        },
        4usize,
        "Offset of field: C_with_zero_length_array_2::zero_length_array",
    );
}
#[repr(C)]
pub struct C_with_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_incomplete_array() {
    const UNINIT: ::std::mem::MaybeUninit<C_with_incomplete_array> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<C_with_incomplete_array>(),
        40usize,
        "Size of C_with_incomplete_array",
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_incomplete_array>(),
        4usize,
        "Alignment of C_with_incomplete_array",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: C_with_incomplete_array::a",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).big_array) as usize - ptr as usize },
        4usize,
        "Offset of field: C_with_incomplete_array::big_array",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).incomplete_array) as usize - ptr as usize },
        37usize,
        "Offset of field: C_with_incomplete_array::incomplete_array",
    );
}
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
#[test]
fn bindgen_test_layout_C_with_incomplete_array_2() {
    const UNINIT: ::std::mem::MaybeUninit<C_with_incomplete_array_2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<C_with_incomplete_array_2>(),
        4usize,
        "Size of C_with_incomplete_array_2",
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_incomplete_array_2>(),
        4usize,
        "Alignment of C_with_incomplete_array_2",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: C_with_incomplete_array_2::a",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).incomplete_array) as usize - ptr as usize },
        4usize,
        "Offset of field: C_with_incomplete_array_2::incomplete_array",
    );
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
    const UNINIT: ::std::mem::MaybeUninit<
        C_with_zero_length_array_and_incomplete_array,
    > = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array_and_incomplete_array>(),
        40usize,
        "Size of C_with_zero_length_array_and_incomplete_array",
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array_and_incomplete_array>(),
        4usize,
        "Alignment of C_with_zero_length_array_and_incomplete_array",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: C_with_zero_length_array_and_incomplete_array::a",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).big_array) as usize - ptr as usize },
        4usize,
        "Offset of field: C_with_zero_length_array_and_incomplete_array::big_array",
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).zero_length_array) as usize - ptr as usize
        },
        37usize,
        "Offset of field: C_with_zero_length_array_and_incomplete_array::zero_length_array",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).incomplete_array) as usize - ptr as usize },
        37usize,
        "Offset of field: C_with_zero_length_array_and_incomplete_array::incomplete_array",
    );
}
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
#[test]
fn bindgen_test_layout_C_with_zero_length_array_and_incomplete_array_2() {
    const UNINIT: ::std::mem::MaybeUninit<
        C_with_zero_length_array_and_incomplete_array_2,
    > = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array_and_incomplete_array_2>(),
        4usize,
        "Size of C_with_zero_length_array_and_incomplete_array_2",
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array_and_incomplete_array_2>(),
        4usize,
        "Alignment of C_with_zero_length_array_and_incomplete_array_2",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: C_with_zero_length_array_and_incomplete_array_2::a",
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).zero_length_array) as usize - ptr as usize
        },
        4usize,
        "Offset of field: C_with_zero_length_array_and_incomplete_array_2::zero_length_array",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).incomplete_array) as usize - ptr as usize },
        4usize,
        "Offset of field: C_with_zero_length_array_and_incomplete_array_2::incomplete_array",
    );
}
#[repr(C)]
#[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct WithDtor {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WithDtor() {
    const UNINIT: ::std::mem::MaybeUninit<WithDtor> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<WithDtor>(), 4usize, "Size of WithDtor");
    assert_eq!(::std::mem::align_of::<WithDtor>(), 4usize, "Alignment of WithDtor");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        "Offset of field: WithDtor::b",
    );
}
#[repr(C)]
pub struct IncompleteArrayNonCopiable {
    pub whatever: *mut ::std::os::raw::c_void,
    pub incomplete_array: __IncompleteArrayField<C>,
}
#[test]
fn bindgen_test_layout_IncompleteArrayNonCopiable() {
    const UNINIT: ::std::mem::MaybeUninit<IncompleteArrayNonCopiable> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<IncompleteArrayNonCopiable>(),
        8usize,
        "Size of IncompleteArrayNonCopiable",
    );
    assert_eq!(
        ::std::mem::align_of::<IncompleteArrayNonCopiable>(),
        8usize,
        "Alignment of IncompleteArrayNonCopiable",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).whatever) as usize - ptr as usize },
        0usize,
        "Offset of field: IncompleteArrayNonCopiable::whatever",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).incomplete_array) as usize - ptr as usize },
        8usize,
        "Offset of field: IncompleteArrayNonCopiable::incomplete_array",
    );
}
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
#[derive(Copy, Clone)]
pub union Union {
    pub d: f32,
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Union() {
    const UNINIT: ::std::mem::MaybeUninit<Union> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<Union>(), 4usize, "Size of Union");
    assert_eq!(::std::mem::align_of::<Union>(), 4usize, "Alignment of Union");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize },
        0usize,
        "Offset of field: Union::d",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).i) as usize - ptr as usize },
        0usize,
        "Offset of field: Union::i",
    );
}
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
#[derive(Copy, Clone)]
pub struct WithUnion {
    pub data: Union,
}
#[test]
fn bindgen_test_layout_WithUnion() {
    const UNINIT: ::std::mem::MaybeUninit<WithUnion> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<WithUnion>(), 4usize, "Size of WithUnion");
    assert_eq!(::std::mem::align_of::<WithUnion>(), 4usize, "Alignment of WithUnion");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        "Offset of field: WithUnion::data",
    );
}
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct RealAbstractionWithTonsOfMethods {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_RealAbstractionWithTonsOfMethods() {
    assert_eq!(
        ::std::mem::size_of::<RealAbstractionWithTonsOfMethods>(),
        1usize,
        "Size of RealAbstractionWithTonsOfMethods",
    );
    assert_eq!(
        ::std::mem::align_of::<RealAbstractionWithTonsOfMethods>(),
        1usize,
        "Alignment of RealAbstractionWithTonsOfMethods",
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
