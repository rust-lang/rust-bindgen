#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
        *self
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
#[derive(Debug, Copy, Clone)]
pub struct StylePoint<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub x: T,
    pub y: T,
}
impl<T> Default for StylePoint<T> {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct StyleFoo<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub __bindgen_anon_1: __BindgenUnionField<StyleFoo__bindgen_ty_1>,
    pub foo: __BindgenUnionField<StyleFoo_Foo_Body<T>>,
    pub bar: __BindgenUnionField<StyleFoo_Bar_Body<T>>,
    pub baz: __BindgenUnionField<StyleFoo_Baz_Body<T>>,
    pub bindgen_union_field: [u8; 0usize],
}
pub const StyleFoo_Tag_Foo: StyleFoo_Tag = 0;
pub const StyleFoo_Tag_Bar: StyleFoo_Tag = 0;
pub const StyleFoo_Tag_Baz: StyleFoo_Tag = 0;
pub const StyleFoo_Tag_Bazz: StyleFoo_Tag = 0;
pub type StyleFoo_Tag = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StyleFoo_Foo_Body<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub tag: StyleFoo_Tag,
    pub x: i32,
    pub y: StylePoint<T>,
    pub z: StylePoint<f32>,
}
impl<T> Default for StyleFoo_Foo_Body<T> {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StyleFoo_Bar_Body<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub tag: StyleFoo_Tag,
    pub _0: T,
}
impl<T> Default for StyleFoo_Bar_Body<T> {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StyleFoo_Baz_Body<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub tag: StyleFoo_Tag,
    pub _0: StylePoint<T>,
}
impl<T> Default for StyleFoo_Baz_Body<T> {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StyleFoo__bindgen_ty_1 {
    pub tag: StyleFoo_Tag,
}
impl Default for StyleFoo__bindgen_ty_1 {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StyleBar<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub tag: StyleBar_Tag,
    pub __bindgen_anon_1: StyleBar__bindgen_ty_1<T>,
}
pub const StyleBar_Tag_Bar1: StyleBar_Tag = 0;
pub const StyleBar_Tag_Bar2: StyleBar_Tag = 0;
pub const StyleBar_Tag_Bar3: StyleBar_Tag = 0;
pub const StyleBar_Tag_Bar4: StyleBar_Tag = 0;
pub type StyleBar_Tag = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StyleBar_StyleBar1_Body<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub x: i32,
    pub y: StylePoint<T>,
    pub z: StylePoint<f32>,
}
impl<T> Default for StyleBar_StyleBar1_Body<T> {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StyleBar_StyleBar2_Body<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _0: T,
}
impl<T> Default for StyleBar_StyleBar2_Body<T> {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StyleBar_StyleBar3_Body<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _0: StylePoint<T>,
}
impl<T> Default for StyleBar_StyleBar3_Body<T> {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct StyleBar__bindgen_ty_1<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub bar1: __BindgenUnionField<StyleBar_StyleBar1_Body<T>>,
    pub bar2: __BindgenUnionField<StyleBar_StyleBar2_Body<T>>,
    pub bar3: __BindgenUnionField<StyleBar_StyleBar3_Body<T>>,
    pub bindgen_union_field: [u8; 0usize],
}
impl<T> Default for StyleBar<T> {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[test]
fn __bindgen_test_layout_StylePoint_open0_float_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<StylePoint<f32>>(),
        8usize,
        "Size of template specialization: StylePoint_open0_float_close0",
    );
    assert_eq!(
        ::std::mem::align_of::<StylePoint<f32>>(),
        4usize,
        "Align of template specialization: StylePoint_open0_float_close0",
    );
}
#[test]
fn __bindgen_test_layout_StylePoint_open0_float_close0_instantiation_1() {
    assert_eq!(
        ::std::mem::size_of::<StylePoint<f32>>(),
        8usize,
        "Size of template specialization: StylePoint_open0_float_close0",
    );
    assert_eq!(
        ::std::mem::align_of::<StylePoint<f32>>(),
        4usize,
        "Align of template specialization: StylePoint_open0_float_close0",
    );
}
