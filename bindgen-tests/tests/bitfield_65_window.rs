#[rustfmt::skip]
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_qualifications)]
#[path = "expectations/tests/issue-3416.rs"]
mod bindings;

#[cfg(target_endian = "little")]
#[test]
fn bitfield_spanning_65_bits() {
    use bindings::{__BindgenBitfieldUnit, S};

    let all = (1u64 << 63) - 1;
    let mut value = S {
        _bitfield_1: S::new_bitfield_1(3, all),
    };
    assert_eq!(value.p(), 3);
    assert_eq!(value.x(), all);
    assert_eq!(unsafe { S::x_raw(&value) }, all);

    value.set_x(0);
    assert_eq!(value.x(), 0);
    assert_eq!(value.p(), 3);
    unsafe { S::set_x_raw(&mut value, all) };
    assert_eq!(value.x(), all);

    let mut unit = __BindgenBitfieldUnit::new([0xff; 9]);
    unit.set(2, 63, 0);
    assert_eq!(unit.get(2, 63), 0);
    for bit in [0, 1, 65, 66, 67, 68, 69, 70, 71] {
        assert!(unit.get_bit(bit), "bit {bit} outside the field changed");
    }
    unsafe { __BindgenBitfieldUnit::raw_set(&mut unit, 2, 63, all) };
    assert_eq!(unsafe { __BindgenBitfieldUnit::raw_get(&unit, 2, 63) }, all);
    assert_eq!(
        unsafe { __BindgenBitfieldUnit::raw_get_const::<2, 63>(&unit) },
        all
    );
}
