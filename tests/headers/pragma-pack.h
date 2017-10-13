
#pragma pack(1)
struct Pack1 {
    unsigned char a;
    int b;
};

#pragma pack(2)
struct Pack2 {
    unsigned char a;
    int b;
};

struct Align {
    unsigned char a;
    int b;
} __attribute__((aligned(8)));
