#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Foo<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub m_member: T,
    pub m_member_ptr: *mut T,
    pub m_member_arr: [T; 1usize],
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub m_member: T,
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
unsafe extern "C" {
    #[link_name = "\u{1}_Z3bar3FooIiiE"]
    pub fn bar(foo: Foo<::std::os::raw::c_int>);
}
#[repr(C)]
#[derive(Debug)]
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
    pub mBConstArray: B<[::std::os::raw::c_int; 1usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 104usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 8usize];
    ["Offset of field: C::mB"][::std::mem::offset_of!(C, mB) - 0usize];
    ["Offset of field: C::mBConstPtr"][::std::mem::offset_of!(C, mBConstPtr) - 8usize];
    [
        "Offset of field: C::mBConstStructPtr",
    ][::std::mem::offset_of!(C, mBConstStructPtr) - 16usize];
    [
        "Offset of field: C::mBConstStructPtrArray",
    ][::std::mem::offset_of!(C, mBConstStructPtrArray) - 24usize];
    ["Offset of field: C::mBConst"][::std::mem::offset_of!(C, mBConst) - 32usize];
    ["Offset of field: C::mBVolatile"][::std::mem::offset_of!(C, mBVolatile) - 36usize];
    [
        "Offset of field: C::mBConstBool",
    ][::std::mem::offset_of!(C, mBConstBool) - 40usize];
    [
        "Offset of field: C::mBConstChar",
    ][::std::mem::offset_of!(C, mBConstChar) - 42usize];
    ["Offset of field: C::mBArray"][::std::mem::offset_of!(C, mBArray) - 44usize];
    ["Offset of field: C::mBPtrArray"][::std::mem::offset_of!(C, mBPtrArray) - 48usize];
    ["Offset of field: C::mBArrayPtr"][::std::mem::offset_of!(C, mBArrayPtr) - 56usize];
    ["Offset of field: C::mBRef"][::std::mem::offset_of!(C, mBRef) - 64usize];
    ["Offset of field: C::mBConstRef"][::std::mem::offset_of!(C, mBConstRef) - 72usize];
    ["Offset of field: C::mPtrRef"][::std::mem::offset_of!(C, mPtrRef) - 80usize];
    ["Offset of field: C::mArrayRef"][::std::mem::offset_of!(C, mArrayRef) - 88usize];
    [
        "Offset of field: C::mBConstArray",
    ][::std::mem::offset_of!(C, mBConstArray) - 96usize];
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
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct D {
    pub m_foo: D_MyFoo,
}
pub type D_MyFoo = Foo<::std::os::raw::c_int>;
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct D_U<Z> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<Z>>,
    pub m_nested_foo: D_MyFoo,
    pub m_baz: Z,
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub prev: *mut T,
    pub next: *mut Rooted<*mut ::std::os::raw::c_void>,
    pub ptr: T,
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RootedContainer"][::std::mem::size_of::<RootedContainer>() - 24usize];
    ["Alignment of RootedContainer"][::std::mem::align_of::<RootedContainer>() - 8usize];
    [
        "Offset of field: RootedContainer::root",
    ][::std::mem::offset_of!(RootedContainer, root) - 0usize];
};
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub member: T,
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PODButContainsDtor"][::std::mem::size_of::<PODButContainsDtor>() - 4usize];
    [
        "Alignment of PODButContainsDtor",
    ][::std::mem::align_of::<PODButContainsDtor>() - 4usize];
    [
        "Offset of field: PODButContainsDtor::member",
    ][::std::mem::offset_of!(PODButContainsDtor, member) - 0usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of POD"][::std::mem::size_of::<POD>() - 4usize];
    ["Alignment of POD"][::std::mem::align_of::<POD>() - 4usize];
    [
        "Offset of field: POD::opaque_member",
    ][::std::mem::offset_of!(POD, opaque_member) - 0usize];
};
/// <div rustbindgen replaces="NestedReplaced"></div>
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NestedReplaced<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub buff: *mut T,
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub buff: *mut T,
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub c: T,
    pub nested: NestedReplaced<T>,
    pub inc: Incomplete<T>,
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub d: T,
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Untemplated"][::std::mem::size_of::<Untemplated>() - 1usize];
    ["Alignment of Untemplated"][::std::mem::align_of::<Untemplated>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Templated {
    pub m_untemplated: Untemplated,
}
/** If the replacement doesn't happen at the parse level the container would be
 copy and the replacement wouldn't, so this wouldn't compile.

 <div rustbindgen replaces="ReplacedWithoutDestructor"></div>*/
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ReplacedWithoutDestructor<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub buff: *mut T,
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub m_member: ReplacedWithoutDestructor<T>,
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
    pub m_member: ReplacedWithoutDestructorFwd<U>,
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
/** If the replacement doesn't happen at the parse level the container would be
 copy and the replacement wouldn't, so this wouldn't compile.

 <div rustbindgen replaces="ReplacedWithoutDestructorFwd"></div>*/
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ReplacedWithoutDestructorFwd<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub buff: *mut T,
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Foo_open0_int_int_close0",
    ][::std::mem::size_of::<Foo<::std::os::raw::c_int>>() - 24usize];
    [
        "Align of template specialization: Foo_open0_int_int_close0",
    ][::std::mem::align_of::<Foo<::std::os::raw::c_int>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_unsigned_int_close0",
    ][::std::mem::size_of::<B<::std::os::raw::c_uint>>() - 4usize];
    [
        "Align of template specialization: B_open0_unsigned_int_close0",
    ][::std::mem::align_of::<B<::std::os::raw::c_uint>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_ptr_const_int_close0",
    ][::std::mem::size_of::<B<*const ::std::os::raw::c_int>>() - 8usize];
    [
        "Align of template specialization: B_open0_ptr_const_int_close0",
    ][::std::mem::align_of::<B<*const ::std::os::raw::c_int>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_ptr_const_mozilla__Foo_close0",
    ][::std::mem::size_of::<B<*const mozilla_Foo>>() - 8usize];
    [
        "Align of template specialization: B_open0_ptr_const_mozilla__Foo_close0",
    ][::std::mem::align_of::<B<*const mozilla_Foo>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_array1_ptr_const_mozilla__Foo_close0",
    ][::std::mem::size_of::<B<[*const mozilla_Foo; 1usize]>>() - 8usize];
    [
        "Align of template specialization: B_open0_array1_ptr_const_mozilla__Foo_close0",
    ][::std::mem::align_of::<B<[*const mozilla_Foo; 1usize]>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_const_int_close0",
    ][::std::mem::size_of::<B<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: B_open0_const_int_close0",
    ][::std::mem::align_of::<B<::std::os::raw::c_int>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_volatile_int_close0",
    ][::std::mem::size_of::<B<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: B_open0_volatile_int_close0",
    ][::std::mem::align_of::<B<::std::os::raw::c_int>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_const_bool_close0",
    ][::std::mem::size_of::<B<bool>>() - 1usize];
    [
        "Align of template specialization: B_open0_const_bool_close0",
    ][::std::mem::align_of::<B<bool>>() - 1usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_const_char16_t_close0",
    ][::std::mem::size_of::<B<u16>>() - 2usize];
    [
        "Align of template specialization: B_open0_const_char16_t_close0",
    ][::std::mem::align_of::<B<u16>>() - 2usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_array1_int_close0",
    ][::std::mem::size_of::<B<[::std::os::raw::c_int; 1usize]>>() - 4usize];
    [
        "Align of template specialization: B_open0_array1_int_close0",
    ][::std::mem::align_of::<B<[::std::os::raw::c_int; 1usize]>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_array1_ptr_int_close0",
    ][::std::mem::size_of::<B<[*mut ::std::os::raw::c_int; 1usize]>>() - 8usize];
    [
        "Align of template specialization: B_open0_array1_ptr_int_close0",
    ][::std::mem::align_of::<B<[*mut ::std::os::raw::c_int; 1usize]>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_ptr_array1_int_close0",
    ][::std::mem::size_of::<B<*mut [::std::os::raw::c_int; 1usize]>>() - 8usize];
    [
        "Align of template specialization: B_open0_ptr_array1_int_close0",
    ][::std::mem::align_of::<B<*mut [::std::os::raw::c_int; 1usize]>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_ref_int_close0",
    ][::std::mem::size_of::<B<*mut ::std::os::raw::c_int>>() - 8usize];
    [
        "Align of template specialization: B_open0_ref_int_close0",
    ][::std::mem::align_of::<B<*mut ::std::os::raw::c_int>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_ref_const_int_close0",
    ][::std::mem::size_of::<B<*const ::std::os::raw::c_int>>() - 8usize];
    [
        "Align of template specialization: B_open0_ref_const_int_close0",
    ][::std::mem::align_of::<B<*const ::std::os::raw::c_int>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_ref_ptr_int_close0",
    ][::std::mem::size_of::<B<*mut *mut ::std::os::raw::c_int>>() - 8usize];
    [
        "Align of template specialization: B_open0_ref_ptr_int_close0",
    ][::std::mem::align_of::<B<*mut *mut ::std::os::raw::c_int>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_ref_array1_int_close0",
    ][::std::mem::size_of::<B<*mut [::std::os::raw::c_int; 1usize]>>() - 8usize];
    [
        "Align of template specialization: B_open0_ref_array1_int_close0",
    ][::std::mem::align_of::<B<*mut [::std::os::raw::c_int; 1usize]>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_array1_const_int_close0",
    ][::std::mem::size_of::<B<[::std::os::raw::c_int; 1usize]>>() - 4usize];
    [
        "Align of template specialization: B_open0_array1_const_int_close0",
    ][::std::mem::align_of::<B<[::std::os::raw::c_int; 1usize]>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Foo_open0_int_int_close0",
    ][::std::mem::size_of::<Foo<::std::os::raw::c_int>>() - 24usize];
    [
        "Align of template specialization: Foo_open0_int_int_close0",
    ][::std::mem::align_of::<Foo<::std::os::raw::c_int>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Rooted_open0_ptr_void_close0",
    ][::std::mem::size_of::<Rooted<*mut ::std::os::raw::c_void>>() - 24usize];
    [
        "Align of template specialization: Rooted_open0_ptr_void_close0",
    ][::std::mem::align_of::<Rooted<*mut ::std::os::raw::c_void>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: Rooted_open0_ptr_void_close0",
    ][::std::mem::size_of::<Rooted<*mut ::std::os::raw::c_void>>() - 24usize];
    [
        "Align of template specialization: Rooted_open0_ptr_void_close0",
    ][::std::mem::align_of::<Rooted<*mut ::std::os::raw::c_void>>() - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: WithDtor_open0_int_close0",
    ][::std::mem::size_of::<WithDtor<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: WithDtor_open0_int_close0",
    ][::std::mem::align_of::<WithDtor<::std::os::raw::c_int>>() - 4usize];
};
