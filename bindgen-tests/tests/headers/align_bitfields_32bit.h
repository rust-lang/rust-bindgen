// bindgen-flags: --no-layout-tests -- --target=i686-unknown-linux-gnu

struct __attribute__((aligned(8))) StructWithBitfieldAndDouble {
    unsigned int bitfield : 32;
    double standard_field;
};
