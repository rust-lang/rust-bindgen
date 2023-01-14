#pragma pack(push, 1)
struct Struct {
    unsigned char a : 1;
    unsigned char b : 1;
    unsigned char c : 6;
    unsigned short int d : 16;
    unsigned char e : 8;
};
#pragma pack(pop)

struct Inner {
    unsigned short a: 16;
    unsigned short b: 16;
};

#pragma pack(push, 1)
struct Outer {
    struct Inner inner;
};
#pragma pop
