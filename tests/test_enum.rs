use bindgen::BindgenOptions;
use support::assert_bind_eq;

fn default_without_rust_enums() -> BindgenOptions {
    BindgenOptions { rust_enums: false, .. Default::default() }
}

#[test]
fn with_simple_enum() {
    assert_bind_eq(Default::default(), "headers/enum.h", "
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum Foo { Bar = 0, Qux = 1, }
        #[derive(Copy, Clone)]
        #[repr(i32)]
        #[derive(Debug)]
        pub enum Neg { MinusOne = -1, One = 1, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/enum.h", "
        pub type Foo = u32;
        pub const Bar: Foo = 0;
        pub const Qux: Foo = 1;
        pub type Neg = i32;
        pub const MinusOne: Neg = -1;
        pub const One: Neg = 1;
    ");
}

#[test]
fn with_packed_enums() {
    assert_bind_eq(Default::default(), "headers/enum_packed.h", "
        #[derive(Copy, Clone)]
        #[repr(u8)]
        #[derive(Debug)]
        pub enum Foo { Bar = 0, Qux = 1, }
        #[derive(Copy, Clone)]
        #[repr(i8)]
        #[derive(Debug)]
        pub enum Neg { MinusOne = -1, One = 1, }
        #[derive(Copy, Clone)]
        #[repr(u16)]
        #[derive(Debug)]
        pub enum Bigger { Much = 255, Larger = 256, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/enum_packed.h", "
        pub type Foo = u8;
        pub const Bar: Foo = 0;
        pub const Qux: Foo = 1;
        pub type Neg = i8;
        pub const MinusOne: Neg = -1;
        pub const One: Neg = 1;
        pub type Bigger = u16;
        pub const Much: Bigger = 255;
        pub const Larger: Bigger = 256;
    ");
}

#[test]
fn with_duplicate_enum_value() {
    assert_bind_eq(Default::default(), "headers/enum_dupe.h", "
        pub const Dupe: Foo = Foo::Bar;
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum Foo { Bar = 1, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/enum_dupe.h", "
        pub type Foo = u32;
        pub const Bar: Foo = 1;
        pub const Dupe: Foo = 1;
    ");
}

#[test]
fn with_explicitly_typed_cxx_enum() {
    assert_bind_eq(Default::default(), "headers/enum_explicit_type.hpp", "
        #[derive(Copy, Clone)]
        #[repr(u8)]
        #[derive(Debug)]
        pub enum Foo { Bar = 0, Qux = 1, }
        #[derive(Copy, Clone)]
        #[repr(i8)]
        #[derive(Debug)]
        pub enum Neg { MinusOne = -1, One = 1, }
        #[derive(Copy, Clone)]
        #[repr(u16)]
        #[derive(Debug)]
        pub enum Bigger { Much = 255, Larger = 256, }
        #[derive(Copy, Clone)]
        #[repr(i64)]
        #[derive(Debug)]
        pub enum MuchLong { MuchLow = -4294967296, }
        #[derive(Copy, Clone)]
        #[repr(u64)]
        #[derive(Debug)]
        pub enum MuchLongLong { MuchHigh = 4294967296, }
        #[derive(Copy, Clone)]
        #[repr(i64)]
        #[derive(Debug)]
        pub enum MostLongLong { MostLow = -9223372036854775808, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/enum_explicit_type.hpp", "
        pub type Foo = u8;
        pub const Bar: Foo = 0;
        pub const Qux: Foo = 1;
        pub type Neg = i8;
        pub const MinusOne: Neg = -1;
        pub const One: Neg = 1;
        pub type Bigger = u16;
        pub const Much: Bigger = 255;
        pub const Larger: Bigger = 256;
        pub type MuchLong = i64;
        pub const MuchLow: MuchLong = -4294967296;
        pub type MuchLongLong = u64;
        pub const MuchHigh: MuchLongLong = 4294967296;
        pub type MostLongLong = i64;
        pub const MostLow: MostLongLong = -9223372036854775808;
    ");
}

#[test]
fn with_overflowed_enum_value() {
    assert_bind_eq(Default::default(), "headers/overflowed_enum.hpp", "
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum Foo {
            BAP_ARM = 9698489,
            BAP_X86 = 11960045,
            BAP_X86_64 = 3128633167,
        }
        #[derive(Copy, Clone)]
        #[repr(u16)]
        #[derive(Debug)]
        pub enum Bar { One = 1, Big = 2, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/overflowed_enum.hpp", "
        pub type Foo = u32;
        pub const BAP_ARM: Foo = 9698489;
        pub const BAP_X86: Foo = 11960045;
        pub const BAP_X86_64: Foo = 3128633167;
        pub type Bar = u16;
        pub const One: Bar = 1;
        pub const Big: Bar = 2;
    ");
}
