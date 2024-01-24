typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;

struct redundant_packed {
    uint32_t a;
    uint32_t b;
} __attribute__((packed, aligned(8)));

struct redundant_packed_bitfield {
    uint8_t a[3];
    uint8_t b0:1;
    uint8_t b1:1;
    uint32_t c;
} __attribute__((packed, aligned(8)));


union redundant_packed_union {
    uint64_t a;
    uint32_t b;
} __attribute__((packed, aligned(16)));


struct inner {
    uint8_t a;
} __attribute__((packed, aligned(2)));

struct outer_redundant_packed {
    struct inner a[2];
    uint32_t b;
} __attribute__((packed, aligned(8)));


#pragma pack(2)

struct redundant_pragma_packed {
    uint8_t a;
    uint16_t b;
} __attribute__((aligned(4)));

#pragma pack()
