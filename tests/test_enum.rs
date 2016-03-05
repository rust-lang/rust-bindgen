use support::assert_bind_eq;

#[test]
fn with_simple_enum() {
    assert_bind_eq("headers/enum.h", "
        #[derive(Clone, Copy)]
        #[repr(u32)]
        pub enum Enum_Foo { Bar = 0, Qux = 1, }
        #[derive(Clone, Copy)]
        #[repr(i32)]
        pub enum Enum_Neg { MinusOne = -1, One = 1, }
    ");
}

#[test]
fn with_packed_enums() {
    assert_bind_eq("headers/enum_packed.h", "
        #[derive(Clone, Copy)]
        #[repr(u8)]
        pub enum Enum_Foo { Bar = 0, Qux = 1, }
        #[derive(Clone, Copy)]
        #[repr(i8)]
        pub enum Enum_Neg { MinusOne = -1, One = 1, }
        #[derive(Clone, Copy)]
        #[repr(u16)]
        pub enum Enum_Bigger { Much = 255, Larger = 256, }
    ");
}

#[test]
fn with_duplicate_enum_value() {
    assert_bind_eq("headers/enum_dupe.h", "
        pub const Dupe: Enum_Foo = Enum_Foo::Bar;
        #[derive(Clone, Copy)]
        #[repr(u32)]
        pub enum Enum_Foo { Bar = 1, }
    ");
}

#[test]
fn with_explicitly_typed_cxx_enum() {
    assert_bind_eq("headers/enum_explicit_type.hpp", "
        #[derive(Clone, Copy)]
        #[repr(u8)]
        pub enum Enum_Foo { Bar = 0, Qux = 1, }
        #[derive(Clone, Copy)]
        #[repr(i8)]
        pub enum Enum_Neg { MinusOne = -1, One = 1, }
        #[derive(Clone, Copy)]
        #[repr(u16)]
        pub enum Enum_Bigger { Much = 255, Larger = 256, }
        #[derive(Clone, Copy)]
        #[repr(i64)]
        pub enum Enum_MuchLong { MuchLow = -4294967296, }
        #[derive(Clone, Copy)]
        #[repr(u64)]
        pub enum Enum_MuchLongLong { MuchHigh = 4294967296, }
    ");
}

#[test]
fn with_overflowed_enum_value() {
    assert_bind_eq("headers/overflowed_enum.hpp", "
        #[derive(Clone, Copy)]
        #[repr(u32)]
        pub enum Enum_Foo {
            BAP_ARM = 9698489,
            BAP_X86 = 11960045,
            BAP_X86_64 = 3128633167,
        }
        #[derive(Clone, Copy)]
        #[repr(u16)]
        pub enum Enum_Bar { One = 1, Big = 2, }
    ");
}
