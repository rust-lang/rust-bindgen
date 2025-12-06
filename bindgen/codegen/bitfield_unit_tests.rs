//! Tests for `__BindgenBitfieldUnit`.
//!
//! Note that bit-fields are allocated right to left (least to most significant
//! bits).
//!
//! From the x86 PS ABI:
//!
//! ```c
//! struct {
//!     int j : 5;
//!     int k : 6;
//!     int m : 7;
//! };
//! ```
//!
//! ```ignore
//! +------------------------------------------------------------+
//! |                     |              |            |          |
//! |       padding       |       m      |     k      |    j     |
//! |31                 18|17          11|10         5|4        0|
//! +------------------------------------------------------------+
//! ```

use super::bitfield_unit::__BindgenBitfieldUnit;

#[test]
fn bitfield_unit_get_bit() {
    let unit = __BindgenBitfieldUnit::<[u8; 2]>::new([0b10011101, 0b00011101]);

    let mut bits = vec![];
    for i in 0..16 {
        bits.push(unit.get_bit(i));
    }

    println!();
    println!("bits = {bits:?}");
    assert_eq!(
        bits,
        &[
            // 0b10011101
            true, false, true, true, true, false, false, true,
            // 0b00011101
            true, false, true, true, true, false, false, false
        ]
    );
}

#[test]
fn bitfield_unit_set_bit() {
    let mut unit =
        __BindgenBitfieldUnit::<[u8; 2]>::new([0b00000000, 0b00000000]);

    for i in 0..16 {
        if i % 3 == 0 {
            unit.set_bit(i, true);
        }
    }

    for i in 0..16 {
        assert_eq!(unit.get_bit(i), i % 3 == 0);
    }

    let mut unit =
        __BindgenBitfieldUnit::<[u8; 2]>::new([0b11111111, 0b11111111]);

    for i in 0..16 {
        if i % 3 == 0 {
            unit.set_bit(i, false);
        }
    }

    for i in 0..16 {
        assert_eq!(unit.get_bit(i), i % 3 != 0);
    }
}

macro_rules! bitfield_unit_get {
    (
        $(
            With $storage:expr , then get($start:expr, $len:expr) is $expected:expr;
        )*
    ) => {
        #[test]
        fn bitfield_unit_get() {
            $({
                let expected = $expected;
                let unit = __BindgenBitfieldUnit::<_>::new($storage);
                let actual = unit.get($start, $len);

                println!();
                println!("expected = {expected:064b}");
                println!("actual   = {actual:064b}");

                assert_eq!(expected, actual);
            })*
        }
   }
}

bitfield_unit_get! {
    // Let's just exhaustively test getting the bits from a single byte, since
    // there are few enough combinations...

    With [0b11100010], then get(0, 1) is 0;
    With [0b11100010], then get(1, 1) is 1;
    With [0b11100010], then get(2, 1) is 0;
    With [0b11100010], then get(3, 1) is 0;
    With [0b11100010], then get(4, 1) is 0;
    With [0b11100010], then get(5, 1) is 1;
    With [0b11100010], then get(6, 1) is 1;
    With [0b11100010], then get(7, 1) is 1;

    With [0b11100010], then get(0, 2) is 0b10;
    With [0b11100010], then get(1, 2) is 0b01;
    With [0b11100010], then get(2, 2) is 0b00;
    With [0b11100010], then get(3, 2) is 0b00;
    With [0b11100010], then get(4, 2) is 0b10;
    With [0b11100010], then get(5, 2) is 0b11;
    With [0b11100010], then get(6, 2) is 0b11;

    With [0b11100010], then get(0, 3) is 0b010;
    With [0b11100010], then get(1, 3) is 0b001;
    With [0b11100010], then get(2, 3) is 0b000;
    With [0b11100010], then get(3, 3) is 0b100;
    With [0b11100010], then get(4, 3) is 0b110;
    With [0b11100010], then get(5, 3) is 0b111;

    With [0b11100010], then get(0, 4) is 0b0010;
    With [0b11100010], then get(1, 4) is 0b0001;
    With [0b11100010], then get(2, 4) is 0b1000;
    With [0b11100010], then get(3, 4) is 0b1100;
    With [0b11100010], then get(4, 4) is 0b1110;

    With [0b11100010], then get(0, 5) is 0b00010;
    With [0b11100010], then get(1, 5) is 0b10001;
    With [0b11100010], then get(2, 5) is 0b11000;
    With [0b11100010], then get(3, 5) is 0b11100;

    With [0b11100010], then get(0, 6) is 0b100010;
    With [0b11100010], then get(1, 6) is 0b110001;
    With [0b11100010], then get(2, 6) is 0b111000;

    With [0b11100010], then get(0, 7) is 0b1100010;
    With [0b11100010], then get(1, 7) is 0b1110001;

    With [0b11100010], then get(0, 8) is 0b11100010;

    // OK. Now let's test getting bits from across byte boundaries.

    With [0b01010101, 0b11111111, 0b00000000, 0b11111111],
    then get(0, 16) is 0b1111111101010101;

    With [0b01010101, 0b11111111, 0b00000000, 0b11111111],
    then get(1, 16) is 0b0111111110101010;

    With [0b01010101, 0b11111111, 0b00000000, 0b11111111],
    then get(2, 16) is 0b0011111111010101;

    With [0b01010101, 0b11111111, 0b00000000, 0b11111111],
    then get(3, 16) is 0b0001111111101010;

    With [0b01010101, 0b11111111, 0b00000000, 0b11111111],
    then get(4, 16) is 0b0000111111110101;

    With [0b01010101, 0b11111111, 0b00000000, 0b11111111],
    then get(5, 16) is 0b0000011111111010;

    With [0b01010101, 0b11111111, 0b00000000, 0b11111111],
    then get(6, 16) is 0b0000001111111101;

    With [0b01010101, 0b11111111, 0b00000000, 0b11111111],
    then get(7, 16) is 0b0000000111111110;

    With [0b01010101, 0b11111111, 0b00000000, 0b11111111],
    then get(8, 16) is 0b0000000011111111;
}

macro_rules! bitfield_unit_set {
    (
        $(
            set($start:expr, $len:expr, $val:expr) is $expected:expr;
        )*
    ) => {
        #[test]
        fn bitfield_unit_set() {
            $(
                let mut unit = __BindgenBitfieldUnit::<[u8; 4]>::new([0, 0, 0, 0]);
                unit.set($start, $len, $val);
                let actual = unit.get(0, 32);

                println!();
                println!("set({}, {}, {:032b}", $start, $len, $val);
                println!("expected = {:064b}", $expected);
                println!("actual   = {actual:064b}");

                assert_eq!($expected, actual);
            )*
        }
    }
}

bitfield_unit_set! {
    // Once again, let's exhaustively test single byte combinations.

    set(0, 1, 0b11111111) is 0b00000001;
    set(1, 1, 0b11111111) is 0b00000010;
    set(2, 1, 0b11111111) is 0b00000100;
    set(3, 1, 0b11111111) is 0b00001000;
    set(4, 1, 0b11111111) is 0b00010000;
    set(5, 1, 0b11111111) is 0b00100000;
    set(6, 1, 0b11111111) is 0b01000000;
    set(7, 1, 0b11111111) is 0b10000000;

    set(0, 2, 0b11111111) is 0b00000011;
    set(1, 2, 0b11111111) is 0b00000110;
    set(2, 2, 0b11111111) is 0b00001100;
    set(3, 2, 0b11111111) is 0b00011000;
    set(4, 2, 0b11111111) is 0b00110000;
    set(5, 2, 0b11111111) is 0b01100000;
    set(6, 2, 0b11111111) is 0b11000000;

    set(0, 3, 0b11111111) is 0b00000111;
    set(1, 3, 0b11111111) is 0b00001110;
    set(2, 3, 0b11111111) is 0b00011100;
    set(3, 3, 0b11111111) is 0b00111000;
    set(4, 3, 0b11111111) is 0b01110000;
    set(5, 3, 0b11111111) is 0b11100000;

    set(0, 4, 0b11111111) is 0b00001111;
    set(1, 4, 0b11111111) is 0b00011110;
    set(2, 4, 0b11111111) is 0b00111100;
    set(3, 4, 0b11111111) is 0b01111000;
    set(4, 4, 0b11111111) is 0b11110000;

    set(0, 5, 0b11111111) is 0b00011111;
    set(1, 5, 0b11111111) is 0b00111110;
    set(2, 5, 0b11111111) is 0b01111100;
    set(3, 5, 0b11111111) is 0b11111000;

    set(0, 6, 0b11111111) is 0b00111111;
    set(1, 6, 0b11111111) is 0b01111110;
    set(2, 6, 0b11111111) is 0b11111100;

    set(0, 7, 0b11111111) is 0b01111111;
    set(1, 7, 0b11111111) is 0b11111110;

    set(0, 8, 0b11111111) is 0b11111111;

    // And, now let's cross byte boundaries.

    set(0, 16, 0b1111111111111111) is 0b00000000000000001111111111111111;
    set(1, 16, 0b1111111111111111) is 0b00000000000000011111111111111110;
    set(2, 16, 0b1111111111111111) is 0b00000000000000111111111111111100;
    set(3, 16, 0b1111111111111111) is 0b00000000000001111111111111111000;
    set(4, 16, 0b1111111111111111) is 0b00000000000011111111111111110000;
    set(5, 16, 0b1111111111111111) is 0b00000000000111111111111111100000;
    set(6, 16, 0b1111111111111111) is 0b00000000001111111111111111000000;
    set(7, 16, 0b1111111111111111) is 0b00000000011111111111111110000000;
    set(8, 16, 0b1111111111111111) is 0b00000000111111111111111100000000;
}

// Tests for const-generic methods
#[test]
fn bitfield_unit_get_const_matches_get() {
    // Test that get_const produces same results as get
    let unit = __BindgenBitfieldUnit::<[u8; 4]>::new([
        0b01010101, 0b11111111, 0b00000000, 0b11111111,
    ]);

    // Single byte tests
    assert_eq!(unit.get_const::<0, 1>(), unit.get(0, 1));
    assert_eq!(unit.get_const::<1, 1>(), unit.get(1, 1));
    assert_eq!(unit.get_const::<0, 8>(), unit.get(0, 8));
    assert_eq!(unit.get_const::<3, 5>(), unit.get(3, 5));

    // Cross-byte boundary tests
    assert_eq!(unit.get_const::<0, 16>(), unit.get(0, 16));
    assert_eq!(unit.get_const::<4, 16>(), unit.get(4, 16));
    assert_eq!(unit.get_const::<7, 16>(), unit.get(7, 16));
    assert_eq!(unit.get_const::<8, 16>(), unit.get(8, 16));

    // Large field
    assert_eq!(unit.get_const::<0, 32>(), unit.get(0, 32));
}

#[test]
fn bitfield_unit_set_const_matches_set() {
    // Test that set_const produces same results as set
    let test_value = 0b101010101010;

    for offset in [0, 1, 3, 7, 8, 12] {
        for width in [1, 2, 5, 8, 12] {
            let mut unit_const = __BindgenBitfieldUnit::<[u8; 4]>::new([0; 4]);
            let mut unit_runtime =
                __BindgenBitfieldUnit::<[u8; 4]>::new([0; 4]);

            match (offset, width) {
                (0, 1) => unit_const.set_const::<0, 1>(test_value),
                (0, 2) => unit_const.set_const::<0, 2>(test_value),
                (0, 5) => unit_const.set_const::<0, 5>(test_value),
                (0, 8) => unit_const.set_const::<0, 8>(test_value),
                (0, 12) => unit_const.set_const::<0, 12>(test_value),
                (1, 1) => unit_const.set_const::<1, 1>(test_value),
                (1, 2) => unit_const.set_const::<1, 2>(test_value),
                (1, 5) => unit_const.set_const::<1, 5>(test_value),
                (1, 8) => unit_const.set_const::<1, 8>(test_value),
                (1, 12) => unit_const.set_const::<1, 12>(test_value),
                (3, 1) => unit_const.set_const::<3, 1>(test_value),
                (3, 2) => unit_const.set_const::<3, 2>(test_value),
                (3, 5) => unit_const.set_const::<3, 5>(test_value),
                (3, 8) => unit_const.set_const::<3, 8>(test_value),
                (3, 12) => unit_const.set_const::<3, 12>(test_value),
                (7, 1) => unit_const.set_const::<7, 1>(test_value),
                (7, 2) => unit_const.set_const::<7, 2>(test_value),
                (7, 5) => unit_const.set_const::<7, 5>(test_value),
                (7, 8) => unit_const.set_const::<7, 8>(test_value),
                (7, 12) => unit_const.set_const::<7, 12>(test_value),
                (8, 1) => unit_const.set_const::<8, 1>(test_value),
                (8, 2) => unit_const.set_const::<8, 2>(test_value),
                (8, 5) => unit_const.set_const::<8, 5>(test_value),
                (8, 8) => unit_const.set_const::<8, 8>(test_value),
                (8, 12) => unit_const.set_const::<8, 12>(test_value),
                (12, 1) => unit_const.set_const::<12, 1>(test_value),
                (12, 2) => unit_const.set_const::<12, 2>(test_value),
                (12, 5) => unit_const.set_const::<12, 5>(test_value),
                (12, 8) => unit_const.set_const::<12, 8>(test_value),
                (12, 12) => unit_const.set_const::<12, 12>(test_value),
                _ => continue,
            }

            unit_runtime.set(offset, width, test_value);
            // Compare by reading back the full value
            assert_eq!(unit_const.get(0, 32), unit_runtime.get(0, 32));
        }
    }
}

#[test]
fn bitfield_unit_raw_const_methods() {
    let unit = __BindgenBitfieldUnit::<[u8; 2]>::new([0b10011101, 0b00011101]);

    // Test raw_get_const
    unsafe {
        assert_eq!(
            __BindgenBitfieldUnit::raw_get_const::<0, 8>(&unit),
            unit.get(0, 8)
        );
        assert_eq!(
            __BindgenBitfieldUnit::raw_get_const::<4, 8>(&unit),
            unit.get(4, 8)
        );
        assert_eq!(
            __BindgenBitfieldUnit::raw_get_const::<0, 16>(&unit),
            unit.get(0, 16)
        );
    }

    // Test raw_set_const
    let mut unit_const = __BindgenBitfieldUnit::<[u8; 2]>::new([0; 2]);
    let mut unit_runtime = __BindgenBitfieldUnit::<[u8; 2]>::new([0; 2]);

    unsafe {
        __BindgenBitfieldUnit::raw_set_const::<3, 5>(&mut unit_const, 0b11111);
    }
    unit_runtime.set(3, 5, 0b11111);

    // Compare by reading back
    assert_eq!(unit_const.get(0, 16), unit_runtime.get(0, 16));
}
