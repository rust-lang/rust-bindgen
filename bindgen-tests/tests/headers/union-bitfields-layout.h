union Union {
    int bitfld_a : 4;
    int bitfld_b : 4;
    int bitfld_c : 4;
};

union WideFirst {
    unsigned int wide : 24;
    unsigned int narrow : 8;
};

union WideLast {
    unsigned int narrow : 8;
    unsigned int wide : 24;
};

union Wide64 {
    unsigned long long wide : 52;
    unsigned long long narrow : 1;
};

union FullWidth {
    unsigned long long wide : 64;
    unsigned long long narrow : 1;
};

union Mixed {
    unsigned long long wide : 52;
    unsigned char bytes[2];
    unsigned int narrow : 1;
};

union MixedScalar {
    float scalar;
    unsigned long long wide : 52;
    unsigned int narrow : 1;
};

union ZeroWidth {
    unsigned int wide : 24;
    unsigned int : 0;
    unsigned int narrow : 8;
};

#pragma pack(push, 1)
union PackedBits {
    unsigned int wide : 24;
    unsigned int narrow : 8;
};

struct PackedContainer {
    unsigned char prefix;
    union PackedBits bits;
    unsigned char suffix;
};

struct PackedNaturalContainer {
    unsigned char prefix;
    union Wide64 bits;
    unsigned char suffix;
};
#pragma pack(pop)
