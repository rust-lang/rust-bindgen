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
        pub enum Enum_Foo { Bar = 0, Qux = 1, }
        #[derive(Copy, Clone)]
        #[repr(i32)]
        #[derive(Debug)]
        pub enum Enum_Neg { MinusOne = -1, One = 1, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/enum.h", "
        type Enum_Foo = u32;
        const Bar: Enum_Foo = 0;
        const Qux: Enum_Foo = 1;
        type Enum_Neg = i32;
        const MinusOne: Enum_Neg = -1;
        const One: Enum_Neg = 1;
    ");
}

#[test]
fn with_packed_enums() {
    assert_bind_eq(Default::default(), "headers/enum_packed.h", "
        #[derive(Copy, Clone)]
        #[repr(u8)]
        #[derive(Debug)]
        pub enum Enum_Foo { Bar = 0, Qux = 1, }
        #[derive(Copy, Clone)]
        #[repr(i8)]
        #[derive(Debug)]
        pub enum Enum_Neg { MinusOne = -1, One = 1, }
        #[derive(Copy, Clone)]
        #[repr(u16)]
        #[derive(Debug)]
        pub enum Enum_Bigger { Much = 255, Larger = 256, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/enum_packed.h", "
        type Enum_Foo = u8;
        const Bar: Enum_Foo = 0;
        const Qux: Enum_Foo = 1;
        type Enum_Neg = i8;
        const MinusOne: Enum_Neg = -1;
        const One: Enum_Neg = 1;
        type Enum_Bigger = u16;
        const Much: Enum_Bigger = 255;
        const Larger: Enum_Bigger = 256;
    ");
}

#[test]
fn with_duplicate_enum_value() {
    assert_bind_eq(Default::default(), "headers/enum_dupe.h", "
        pub const Dupe: Enum_Foo = Enum_Foo::Bar;
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum Enum_Foo { Bar = 1, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/enum_dupe.h", "
        type Enum_Foo = u32;
        const Bar: Enum_Foo = 1;
        const Dupe: Enum_Foo = 1;
    ");
}

#[test]
fn with_explicitly_typed_cxx_enum() {
    assert_bind_eq(Default::default(), "headers/enum_explicit_type.hpp", "
        #[derive(Copy, Clone)]
        #[repr(u8)]
        #[derive(Debug)]
        pub enum Enum_Foo { Bar = 0, Qux = 1, }
        #[derive(Copy, Clone)]
        #[repr(i8)]
        #[derive(Debug)]
        pub enum Enum_Neg { MinusOne = -1, One = 1, }
        #[derive(Copy, Clone)]
        #[repr(u16)]
        #[derive(Debug)]
        pub enum Enum_Bigger { Much = 255, Larger = 256, }
        #[derive(Copy, Clone)]
        #[repr(i64)]
        #[derive(Debug)]
        pub enum Enum_MuchLong { MuchLow = -4294967296, }
        #[derive(Copy, Clone)]
        #[repr(u64)]
        #[derive(Debug)]
        pub enum Enum_MuchLongLong { MuchHigh = 4294967296, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/enum_explicit_type.hpp", "
        type Enum_Foo = u8;
        const Bar: Enum_Foo = 0;
        const Qux: Enum_Foo = 1;
        type Enum_Neg = i8;
        const MinusOne: Enum_Neg = -1;
        const One: Enum_Neg = 1;
        type Enum_Bigger = u16;
        const Much: Enum_Bigger = 255;
        const Larger: Enum_Bigger = 256;
        type Enum_MuchLong = i64;
        const MuchLow: Enum_MuchLong = -4294967296;
        type Enum_MuchLongLong = u64;
        const MuchHigh: Enum_MuchLongLong = 4294967296;
    ");
}

#[test]
fn with_overflowed_enum_value() {
    assert_bind_eq(Default::default(), "headers/overflowed_enum.hpp", "
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum Enum_Foo {
            BAP_ARM = 9698489,
            BAP_X86 = 11960045,
            BAP_X86_64 = 3128633167,
        }
        #[derive(Copy, Clone)]
        #[repr(u16)]
        #[derive(Debug)]
        pub enum Enum_Bar { One = 1, Big = 2, }
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/overflowed_enum.hpp", "
        type Enum_Foo = u32;
        const BAP_ARM: Enum_Foo = 9698489;
        const BAP_X86: Enum_Foo = 11960045;
        const BAP_X86_64: Enum_Foo = 3128633167;
        type Enum_Bar = u16;
        const One: Enum_Bar = 1;
        const Big: Enum_Bar = 2;
    ");
}
