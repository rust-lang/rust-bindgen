#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
pub use self::foo::Type as foo_alias1;
pub use self::foo_alias1 as foo_alias2;
pub use self::foo_alias2 as foo_alias3;
pub use self::anon_enum::Type as anon_enum_alias1;
pub use self::anon_enum_alias1 as anon_enum_alias2;
pub use self::anon_enum_alias2 as anon_enum_alias3;
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
const _: () = {
    ["Size of bar"][::std::mem::size_of::<bar>() - 48usize];
    ["Alignment of bar"][::std::mem::align_of::<bar>() - 8usize];
    ["Offset of field: bar::member1"][::std::mem::offset_of!(bar, member1) - 0usize];
    ["Offset of field: bar::member2"][::std::mem::offset_of!(bar, member2) - 4usize];
    ["Offset of field: bar::member3"][::std::mem::offset_of!(bar, member3) - 8usize];
    ["Offset of field: bar::member4"][::std::mem::offset_of!(bar, member4) - 12usize];
    ["Offset of field: bar::member5"][::std::mem::offset_of!(bar, member5) - 16usize];
    ["Offset of field: bar::member6"][::std::mem::offset_of!(bar, member6) - 24usize];
    ["Offset of field: bar::member7"][::std::mem::offset_of!(bar, member7) - 32usize];
    ["Offset of field: bar::member8"][::std::mem::offset_of!(bar, member8) - 36usize];
    ["Offset of field: bar::member9"][::std::mem::offset_of!(bar, member9) - 40usize];
    ["Offset of field: bar::member10"][::std::mem::offset_of!(bar, member10) - 44usize];
};
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
const _: () = {
    ["Size of Baz"][::std::mem::size_of::<Baz>() - 4usize];
    ["Alignment of Baz"][::std::mem::align_of::<Baz>() - 4usize];
    ["Offset of field: Baz::member1"][::std::mem::offset_of!(Baz, member1) - 0usize];
};
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
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 8usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 8usize];
    ["Offset of field: Bar::baz"][::std::mem::offset_of!(Bar, baz) - 0usize];
};
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
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub thing: T,
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
