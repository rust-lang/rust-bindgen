#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub mod foo {
    pub type Type = ::std::os::raw::c_uint;
    pub const THIS: Type = 0;
    pub const SHOULD_BE: Type = 1;
    pub const A_CONSTANT: Type = 2;
    pub const ALSO_THIS: Type = 42;
    pub const AND_ALSO_THIS: Type = 42;
}
pub mod anon_enum {
    pub type Type = ::std::os::raw::c_uint;
    pub const Variant1: Type = 0;
    pub const Variant2: Type = 1;
    pub const Variant3: Type = 2;
}
pub mod ns1_foo {
    pub type Type = ::std::os::raw::c_uint;
    pub const THIS: Type = 0;
    pub const SHOULD_BE: Type = 1;
    pub const A_CONSTANT: Type = 2;
    pub const ALSO_THIS: Type = 42;
}
pub mod ns2_Foo {
    pub type Type = ::std::os::raw::c_int;
    pub const Variant1: Type = 0;
    pub const Variant2: Type = 1;
}
pub use self::anon_enum::Type as anon_enum_alias1;
pub use self::anon_enum_alias1 as anon_enum_alias2;
pub use self::anon_enum_alias2 as anon_enum_alias3;
pub use self::foo::Type as foo_alias1;
pub use self::foo_alias1 as foo_alias2;
pub use self::foo_alias2 as foo_alias3;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bar {
    pub member1: foo::Type,
    pub member2: foo_alias1,
    pub member3: foo_alias2,
    pub member4: foo_alias3,
    pub member5: ns1_foo::Type,
    pub member6: *mut ns2_Foo::Type,
    pub member7: anon_enum::Type,
    pub member8: anon_enum_alias1,
    pub member9: anon_enum_alias2,
    pub member10: anon_enum_alias3,
}
#[test]
fn bindgen_test_layout_bar() {
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        48usize,
        concat!("Size of: ", stringify!(bar))
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        8usize,
        concat!("Alignment of ", stringify!(bar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member2 as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member3 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member4 as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member5 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member5)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member6 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member6)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member7 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member7)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member8 as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).member9 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bar>())).member10 as *const _ as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(member10)
        )
    );
}
impl Default for bar {
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
pub struct Baz {
    pub member1: ns2_Foo::Type,
}
#[test]
fn bindgen_test_layout_Baz() {
    assert_eq!(
        ::std::mem::size_of::<Baz>(),
        4usize,
        concat!("Size of: ", stringify!(Baz))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz>(),
        4usize,
        concat!("Alignment of ", stringify!(Baz))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Baz>())).member1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Baz),
            "::",
            stringify!(member1)
        )
    );
}
impl Default for Baz {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub mod one_Foo {
    pub type Type = ::std::os::raw::c_int;
    pub const Variant1: Type = 0;
    pub const Variant2: Type = 1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bar {
    pub baz: *mut one_Foo::Type,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        8usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        8usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Bar>())).baz as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(baz))
    );
}
impl Default for Bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_Z5func13fooPS_PS0_"]
    pub fn func1(
        arg1: foo::Type,
        arg2: *mut foo::Type,
        arg3: *mut *mut foo::Type,
    ) -> *mut foo::Type;
}
extern "C" {
    #[link_name = "\u{1}_Z5func23fooPS_PS0_"]
    pub fn func2(
        arg1: foo_alias1,
        arg2: *mut foo_alias1,
        arg3: *mut *mut foo_alias1,
    ) -> *mut foo_alias1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Thing<T> {
    pub thing: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Thing<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_Z5func35ThingI3fooE"]
    pub fn func3(arg1: Thing<foo::Type>) -> foo::Type;
}
extern "C" {
    #[link_name = "\u{1}_Z5func45ThingIS_I3fooEE"]
    pub fn func4(arg1: Thing<Thing<foo::Type>>) -> foo::Type;
}
