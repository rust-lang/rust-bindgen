/// Bitfield accessor name conflicts:
/// - `set_x` getter collides with `x` setter
/// - `set_x_bindgen_bitfield` tests the collision chain (the suffix
///   used for deduplication itself collides with a real field name)
struct test {
    char set_x: 1;
    char x: 1;
    char set_x_bindgen_bitfield: 1;
};
