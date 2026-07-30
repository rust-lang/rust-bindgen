#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StylePoint<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub x: T,
    pub y: T,
}
impl<T> Default for StylePoint<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub union StyleFoo<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub __bindgen_anon_1: ::std::mem::ManuallyDrop<StyleFoo__bindgen_ty_1>,
    pub foo: ::std::mem::ManuallyDrop<StyleFoo_Foo_Body<T>>,
    pub bar: ::std::mem::ManuallyDrop<StyleFoo_Bar_Body<T>>,
    pub baz: ::std::mem::ManuallyDrop<StyleFoo_Baz_Body<T>>,
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl<T> Default for StyleFoo<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(1))]
pub union StyleBar__bindgen_ty_1<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub bar1: ::std::mem::ManuallyDrop<StyleBar_StyleBar1_Body<T>>,
    pub bar2: ::std::mem::ManuallyDrop<StyleBar_StyleBar2_Body<T>>,
    pub bar3: ::std::mem::ManuallyDrop<StyleBar_StyleBar3_Body<T>>,
}
impl<T> Default for StyleBar__bindgen_ty_1<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl<T> Default for StyleBar<T> {
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
        "Size of template specialization: StylePoint_open0_float_close0",
    ][::std::mem::size_of::<StylePoint<f32>>() - 8usize];
    [
        "Align of template specialization: StylePoint_open0_float_close0",
    ][::std::mem::align_of::<StylePoint<f32>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: StylePoint_open0_float_close0",
    ][::std::mem::size_of::<StylePoint<f32>>() - 8usize];
    [
        "Align of template specialization: StylePoint_open0_float_close0",
    ][::std::mem::align_of::<StylePoint<f32>>() - 4usize];
};
