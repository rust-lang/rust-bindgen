mod bindings {
    include!(concat!(env!("OUT_DIR"), "/union-bitfields.rs"));
}

use bindings::*;
use std::mem::{size_of, zeroed};
use std::ptr::addr_of_mut;

#[test]
fn overlapping_bitfields_share_storage() {
    unsafe {
        assert_eq!(size_of::<Union>(), union_bits_size() as usize);
        let mut value: Union = zeroed();
        value.set_bitfld_a(3);
        assert_eq!(value.bitfld_b(), 3);
        assert_eq!(value.bitfld_c(), 3);
        assert_eq!(union_bits_get_b(&value), 3);
        union_bits_set_c(&mut value, 7);
        assert_eq!(value.bitfld_a(), 7);
        assert_eq!(value.bitfld_b(), 7);
        assert_eq!(Union::bitfld_c_raw(&value), 7);
        Union::set_bitfld_b_raw(&mut value, 2);
        assert_eq!(union_bits_get_b(&value), 2);
    }
}

#[test]
fn widest_bitfield_is_not_last() {
    unsafe {
        assert_eq!(size_of::<WideFirst>(), wide_first_size() as usize);
        let mut value: WideFirst = zeroed();
        value.set_wide(0xabcdef);
        assert_eq!(value.wide(), wide_first_get_wide(&value));
        assert_eq!(value.narrow(), wide_first_get_narrow(&value));
        wide_first_set_wide(&mut value, 0x123456);
        assert_eq!(value.wide(), 0x123456);
        assert_eq!(WideFirst::wide_raw(&value), 0x123456);
        WideFirst::set_wide_raw(&mut value, 0xfedcba);
        assert_eq!(wide_first_get_wide(&value), 0xfedcba);
    }
}

#[test]
fn widest_bitfield_uses_more_than_four_bytes() {
    unsafe {
        assert_eq!(size_of::<Wide64>(), wide64_size() as usize);
        let mut value: Wide64 = zeroed();
        value.set_wide(0xabcdef0123456);
        assert_eq!(wide64_get_wide(&value), 0xabcdef0123456);
        wide64_set_wide(&mut value, 0x123456789abcd);
        assert_eq!(value.wide(), 0x123456789abcd);
        assert_eq!(Wide64::wide_raw(&value), 0x123456789abcd);
    }
}

#[test]
fn field_order_and_zero_width_do_not_shrink_storage() {
    unsafe {
        let mut last: WideLast = zeroed();
        last.set_wide(0xabcdef);
        assert_eq!(last.wide(), 0xabcdef);
        let mut zero_width: ZeroWidth = zeroed();
        zero_width.set_wide(0xabcdef);
        assert_eq!(zero_width.wide(), 0xabcdef);
        let mut mixed: Mixed = zeroed();
        mixed.set_wide(0xabcdef0123456);
        assert_eq!(mixed.wide(), 0xabcdef0123456);
        assert_eq!(size_of::<WideFirst>(), size_of::<WideLast>());
    }
}

#[test]
fn unions_cross_the_c_abi_by_value() {
    unsafe {
        let mut value: WideFirst = zeroed();
        value.set_wide(0xabcdef);
        let result = wide_first_by_value(value, 0x123456);
        assert_eq!(result.wide(), 0xabcdef ^ 0x123456);

        let mut mixed: MixedScalar = zeroed();
        mixed.set_wide(0xabcdef0123456);
        let result = mixed_by_value(mixed, 0x123456789abcd);
        assert_eq!(result.wide(), 0xabcdef0123456 ^ 0x123456789abcd);
    }
}

#[test]
fn naturally_aligned_union_in_packed_container() {
    unsafe {
        let mut container: PackedNaturalContainer = zeroed();
        container.prefix = 0x12;
        container.suffix = 0x34;
        let bits = addr_of_mut!(container.bits);
        Wide64::set_wide_raw(bits, 0xabcdef0123456);
        assert_eq!(Wide64::wide_raw(bits), 0xabcdef0123456);
        assert_eq!(container.prefix, 0x12);
        assert_eq!(container.suffix, 0x34);
    }
}

#[test]
fn full_width_bitfield_preserves_all_bits() {
    unsafe {
        let mut value: FullWidth = zeroed();
        value.set_wide(u64::MAX);
        assert_eq!(value.wide(), u64::MAX);
        assert_eq!(FullWidth::wide_raw(&value), u64::MAX);
    }
}
