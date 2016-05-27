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
        type Foo = u32;
        const Bar: Foo = 0;
        const Qux: Foo = 1;
        type Neg = i32;
        const MinusOne: Neg = -1;
        const One: Neg = 1;
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
        type Foo = u8;
        const Bar: Foo = 0;
        const Qux: Foo = 1;
        type Neg = i8;
        const MinusOne: Neg = -1;
        const One: Neg = 1;
        type Bigger = u16;
        const Much: Bigger = 255;
        const Larger: Bigger = 256;
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
        type Foo = u32;
        const Bar: Foo = 1;
        const Dupe: Foo = 1;
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
    ");
    assert_bind_eq(default_without_rust_enums(), "headers/enum_explicit_type.hpp", "
        type Foo = u8;
        const Bar: Foo = 0;
        const Qux: Foo = 1;
        type Neg = i8;
        const MinusOne: Neg = -1;
        const One: Neg = 1;
        type Bigger = u16;
        const Much: Bigger = 255;
        const Larger: Bigger = 256;
        type MuchLong = i64;
        const MuchLow: MuchLong = -4294967296;
        type MuchLongLong = u64;
        const MuchHigh: MuchLongLong = 4294967296;
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
        type Foo = u32;
        const BAP_ARM: Foo = 9698489;
        const BAP_X86: Foo = 11960045;
        const BAP_X86_64: Foo = 3128633167;
        type Bar = u16;
        const One: Bar = 1;
        const Big: Bar = 2;
    ");
}
