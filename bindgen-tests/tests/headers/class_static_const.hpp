// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --keep-integer-radices
using int32_t = int;
typedef unsigned int uint32_t;

class A {
    static const int a = 0;
    static const int32_t b = 077;
    static const uint32_t c = 0xff;
};
