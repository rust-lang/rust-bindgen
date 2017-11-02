// bindgen-flags: --no-layout-tests
//
// Unfortunately, we aren't translating the second and third structs correctly
// yet. But we definitely shouldn't divide-by-zero when we see it...
//
// Once we fix #981 we should remove the `--no-layout-tests`.

struct WithBitfield {
    unsigned : 7;
    unsigned a;
};

struct WithBitfieldAndAttrPacked {
    unsigned : 7;
    unsigned a;
} __attribute__((packed));

#pragma pack(1)
struct WithBitfieldAndPacked {
    unsigned : 7;
    unsigned a;
};
