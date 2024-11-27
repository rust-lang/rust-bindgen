// bindgen-flags: \-\-rust-target=1.33
//
// FIXME: https://github.com/rust-lang/rust-bindgen/issues/1498


#if 0
struct header
{
    char proto;
    unsigned int size __attribute__ ((packed));
    unsigned char data[] __attribute__ ((aligned(8)));
} __attribute__ ((aligned, packed));
#endif
