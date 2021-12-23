#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Foo<T> {
    pub m_member: T,
    pub m_member_ptr: *mut T,
    pub m_member_arr: [T; 1usize],
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Foo<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B<T> {
    pub m_member: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for B<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_Z3bar3FooIiiE"]
    pub fn bar(foo: Foo<::std::os::raw::c_int>);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mozilla_Foo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct C {
    pub mB: B<::std::os::raw::c_uint>,
    pub mBConstPtr: B<*const ::std::os::raw::c_int>,
    pub mBConstStructPtr: B<*const mozilla_Foo>,
    pub mBConstStructPtrArray: B<[*const mozilla_Foo; 1usize]>,
    pub mBConst: B<::std::os::raw::c_int>,
    pub mBVolatile: B<::std::os::raw::c_int>,
    pub mBConstBool: B<bool>,
    pub mBConstChar: B<u16>,
    pub mBArray: B<[::std::os::raw::c_int; 1usize]>,
    pub mBPtrArray: B<[*mut ::std::os::raw::c_int; 1usize]>,
    pub mBArrayPtr: B<*mut [::std::os::raw::c_int; 1usize]>,
    pub mBRef: B<*mut ::std::os::raw::c_int>,
    pub mBConstRef: B<*const ::std::os::raw::c_int>,
    pub mPtrRef: B<*mut *mut ::std::os::raw::c_int>,
    pub mArrayRef: B<*mut [::std::os::raw::c_int; 1usize]>,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        96usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        8usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).mB as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(mB))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C>())).mBConstPtr as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBConstPtr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C>())).mBConstStructPtr as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBConstStructPtr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C>())).mBConstStructPtrArray as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBConstStructPtrArray)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).mBConst as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBConst)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C>())).mBVolatile as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBVolatile)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C>())).mBConstBool as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBConstBool)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C>())).mBConstChar as *const _ as usize
        },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBConstChar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).mBArray as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBArray)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C>())).mBPtrArray as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBPtrArray)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C>())).mBArrayPtr as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBArrayPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).mBRef as *const _ as usize },
        64usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(mBRef))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<C>())).mBConstRef as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mBConstRef)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).mPtrRef as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mPtrRef)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).mArrayRef as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(mArrayRef)
        )
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
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct D {
    pub m_foo: D_MyFoo,
}
pub type D_MyFoo = Foo<::std::os::raw::c_int>;
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct D_U<Z> {
    pub m_nested_foo: D_MyFoo,
    pub m_baz: Z,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<Z>>,
}
impl<Z> Default for D_U<Z> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for D {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Rooted<T> {
    pub prev: *mut T,
    pub next: *mut Rooted<*mut ::std::os::raw::c_void>,
    pub ptr: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Rooted<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct RootedContainer {
    pub root: Rooted<*mut ::std::os::raw::c_void>,
}
#[test]
fn bindgen_test_layout_RootedContainer() {
    assert_eq!(
        ::std::mem::size_of::<RootedContainer>(),
        24usize,
        concat!("Size of: ", stringify!(RootedContainer))
    );
    assert_eq!(
        ::std::mem::align_of::<RootedContainer>(),
        8usize,
        concat!("Alignment of ", stringify!(RootedContainer))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<RootedContainer>())).root as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RootedContainer),
            "::",
            stringify!(root)
        )
    );
}
impl Default for RootedContainer {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type WithDtorIntFwd = WithDtor<::std::os::raw::c_int>;
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct WithDtor<T> {
    pub member: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for WithDtor<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PODButContainsDtor {
    pub member: WithDtorIntFwd,
}
#[test]
fn bindgen_test_layout_PODButContainsDtor() {
    assert_eq!(
        ::std::mem::size_of::<PODButContainsDtor>(),
        4usize,
        concat!("Size of: ", stringify!(PODButContainsDtor))
    );
    assert_eq!(
        ::std::mem::align_of::<PODButContainsDtor>(),
        4usize,
        concat!("Alignment of ", stringify!(PODButContainsDtor))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<PODButContainsDtor>())).member as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PODButContainsDtor),
            "::",
            stringify!(member)
        )
    );
}
impl Default for PODButContainsDtor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// <div rustbindgen opaque>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Opaque {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct POD {
    pub opaque_member: u32,
}
#[test]
fn bindgen_test_layout_POD() {
    assert_eq!(
        ::std::mem::size_of::<POD>(),
        4usize,
        concat!("Size of: ", stringify!(POD))
    );
    assert_eq!(
        ::std::mem::align_of::<POD>(),
        4usize,
        concat!("Alignment of ", stringify!(POD))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<POD>())).opaque_member as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(POD),
            "::",
            stringify!(opaque_member)
        )
    );
}
/// <div rustbindgen replaces="NestedReplaced"></div>
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NestedReplaced<T> {
    pub buff: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for NestedReplaced<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NestedBase<T> {
    pub buff: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for NestedBase<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NestedContainer<T> {
    pub c: T,
    pub nested: NestedReplaced<T>,
    pub inc: Incomplete<T>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for NestedContainer<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Incomplete<T> {
    pub d: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Incomplete<T> {
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
pub struct Untemplated {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Untemplated() {
    assert_eq!(
        ::std::mem::size_of::<Untemplated>(),
        1usize,
        concat!("Size of: ", stringify!(Untemplated))
    );
    assert_eq!(
        ::std::mem::align_of::<Untemplated>(),
        1usize,
        concat!("Alignment of ", stringify!(Untemplated))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Templated {
    pub m_untemplated: Untemplated,
}
/// If the replacement doesn't happen at the parse level the container would be
/// copy and the replacement wouldn't, so this wouldn't compile.
///
/// <div rustbindgen replaces="ReplacedWithoutDestructor"></div>
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ReplacedWithoutDestructor<T> {
    pub buff: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for ReplacedWithoutDestructor<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ShouldNotBeCopiable<T> {
    pub m_member: ReplacedWithoutDestructor<T>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for ShouldNotBeCopiable<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ShouldNotBeCopiableAsWell<U> {
    pub m_member: ReplacedWithoutDestructorFwd<U>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
impl<U> Default for ShouldNotBeCopiableAsWell<U> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// If the replacement doesn't happen at the parse level the container would be
/// copy and the replacement wouldn't, so this wouldn't compile.
///
/// <div rustbindgen replaces="ReplacedWithoutDestructorFwd"></div>
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ReplacedWithoutDestructorFwd<T> {
    pub buff: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for ReplacedWithoutDestructorFwd<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn __bindgen_test_layout_Foo_open0_int_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Foo<::std::os::raw::c_int>>(),
        24usize,
        concat!(
            "Size of template specialization: ",
            stringify!(Foo<::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Foo<::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(Foo<::std::os::raw::c_int>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_unsigned_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<::std::os::raw::c_uint>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<::std::os::raw::c_uint>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<::std::os::raw::c_uint>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<::std::os::raw::c_uint>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_ptr_const_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<*const ::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<*const ::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<*const ::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<*const ::std::os::raw::c_int>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_ptr_const_mozilla__Foo_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<*const mozilla_Foo>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<*const mozilla_Foo>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<*const mozilla_Foo>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<*const mozilla_Foo>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_array1_ptr_const_mozilla__Foo_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<B<[*const mozilla_Foo; 1usize]>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<[*const mozilla_Foo; 1usize]>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<[*const mozilla_Foo; 1usize]>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<[*const mozilla_Foo; 1usize]>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_const_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<::std::os::raw::c_int>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_volatile_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<::std::os::raw::c_int>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_const_bool_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<bool>>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(B<bool>))
    );
    assert_eq!(
        ::std::mem::align_of::<B<bool>>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<bool>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_const_char16_t_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<u16>>(),
        2usize,
        concat!("Size of template specialization: ", stringify!(B<u16>))
    );
    assert_eq!(
        ::std::mem::align_of::<B<u16>>(),
        2usize,
        concat!("Alignment of template specialization: ", stringify!(B<u16>))
    );
}
#[test]
fn __bindgen_test_layout_B_open0_array1_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<[::std::os::raw::c_int; 1usize]>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<[::std::os::raw::c_int; 1usize]>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<[::std::os::raw::c_int; 1usize]>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<[::std::os::raw::c_int; 1usize]>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_array1_ptr_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<[*mut ::std::os::raw::c_int; 1usize]>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<[*mut ::std::os::raw::c_int; 1usize]>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<[*mut ::std::os::raw::c_int; 1usize]>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<[*mut ::std::os::raw::c_int; 1usize]>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_ptr_array1_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<*mut [::std::os::raw::c_int; 1usize]>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<*mut [::std::os::raw::c_int; 1usize]>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<*mut [::std::os::raw::c_int; 1usize]>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<*mut [::std::os::raw::c_int; 1usize]>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_ref_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<*mut ::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<*mut ::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<*mut ::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<*mut ::std::os::raw::c_int>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_ref_const_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<*const ::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<*const ::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<*const ::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<*const ::std::os::raw::c_int>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_ref_ptr_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<*mut *mut ::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<*mut *mut ::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<*mut *mut ::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<*mut *mut ::std::os::raw::c_int>)
        )
    );
}
#[test]
fn __bindgen_test_layout_B_open0_ref_array1_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<B<*mut [::std::os::raw::c_int; 1usize]>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(B<*mut [::std::os::raw::c_int; 1usize]>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<B<*mut [::std::os::raw::c_int; 1usize]>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(B<*mut [::std::os::raw::c_int; 1usize]>)
        )
    );
}
#[test]
fn __bindgen_test_layout_Foo_open0_int_int_close0_instantiation_1() {
    assert_eq!(
        ::std::mem::size_of::<Foo<::std::os::raw::c_int>>(),
        24usize,
        concat!(
            "Size of template specialization: ",
            stringify!(Foo<::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Foo<::std::os::raw::c_int>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(Foo<::std::os::raw::c_int>)
        )
    );
}
#[test]
fn __bindgen_test_layout_Rooted_open0_ptr_void_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Rooted<*mut ::std::os::raw::c_void>>(),
        24usize,
        concat!(
            "Size of template specialization: ",
            stringify!(Rooted<*mut ::std::os::raw::c_void>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Rooted<*mut ::std::os::raw::c_void>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(Rooted<*mut ::std::os::raw::c_void>)
        )
    );
}
#[test]
fn __bindgen_test_layout_Rooted_open0_ptr_void_close0_instantiation_1() {
    assert_eq!(
        ::std::mem::size_of::<Rooted<*mut ::std::os::raw::c_void>>(),
        24usize,
        concat!(
            "Size of template specialization: ",
            stringify!(Rooted<*mut ::std::os::raw::c_void>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Rooted<*mut ::std::os::raw::c_void>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(Rooted<*mut ::std::os::raw::c_void>)
        )
    );
}
#[test]
fn __bindgen_test_layout_WithDtor_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<WithDtor<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Size of template specialization: ",
            stringify!(WithDtor<::std::os::raw::c_int>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<WithDtor<::std::os::raw::c_int>>(),
        4usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(WithDtor<::std::os::raw::c_int>)
        )
    );
}
