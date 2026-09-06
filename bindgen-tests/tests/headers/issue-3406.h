struct __attribute__((aligned(16))) Inner {
    char byte;
};

typedef int AlignedInt __attribute__((aligned(16)));
typedef AlignedInt NestedAlignedInt;

struct Outer {
    int before;
    struct Inner inner;
};

struct Outer2 {
    int before;
    AlignedInt one;
    AlignedInt two;
};

struct Outer3 {
    int before;
    AlignedInt inner;
};

struct Outer4 {
    int before;
    NestedAlignedInt inner;
};
