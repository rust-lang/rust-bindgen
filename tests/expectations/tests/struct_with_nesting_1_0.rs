#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct foo {
    pub a: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1 {
    pub b: __BindgenUnionField<::std::os::raw::c_uint>,
    pub __bindgen_anon_1: __BindgenUnionField<foo__bindgen_ty_1__bindgen_ty_1>,
    pub __bindgen_anon_2: __BindgenUnionField<foo__bindgen_ty_1__bindgen_ty_2>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_1 {
    pub c1: ::std::os::raw::c_ushort,
    pub c2: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<foo__bindgen_ty_1__bindgen_ty_1>()
            };
            let struct_ptr =
                &struct_instance as *const foo__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.c1);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(c1)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<foo__bindgen_ty_1__bindgen_ty_1>()
            };
            let struct_ptr =
                &struct_instance as *const foo__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.c2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(c2)
        )
    );
}
impl Clone for foo__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_2 {
    pub d1: ::std::os::raw::c_uchar,
    pub d2: ::std::os::raw::c_uchar,
    pub d3: ::std::os::raw::c_uchar,
    pub d4: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<foo__bindgen_ty_1__bindgen_ty_2>()
            };
            let struct_ptr =
                &struct_instance as *const foo__bindgen_ty_1__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.d1);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(d1)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<foo__bindgen_ty_1__bindgen_ty_2>()
            };
            let struct_ptr =
                &struct_instance as *const foo__bindgen_ty_1__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.d2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(d2)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<foo__bindgen_ty_1__bindgen_ty_2>()
            };
            let struct_ptr =
                &struct_instance as *const foo__bindgen_ty_1__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.d3);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(d3)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<foo__bindgen_ty_1__bindgen_ty_2>()
            };
            let struct_ptr =
                &struct_instance as *const foo__bindgen_ty_1__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.d4);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(d4)
        )
    );
}
impl Clone for foo__bindgen_ty_1__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        8usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<foo>() };
            let struct_ptr = &struct_instance as *const foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
    );
}
impl Clone for foo {
    fn clone(&self) -> Self {
        *self
    }
}
