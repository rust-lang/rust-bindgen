// bindgen-flags: --rust-target 1.21
//
// FIXME: https://github.com/rust-lang/rust-bindgen/issues/1498

struct header
{
    char proto;
    unsigned int size __attribute__ ((packed));
    unsigned char data[] __attribute__ ((aligned(8)));
} __attribute__ ((aligned, packed));
