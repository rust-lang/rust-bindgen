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

struct Inner2 { long long a, b; };
typedef struct Inner2 AlignedInner2 __attribute__((aligned(16)));

struct Outer5 {
    long long before;
    AlignedInner2 inner[1];
};

struct Outer6 {
    long long before;
    AlignedInner2 inner[0];
    char tail;
};

enum __attribute__((aligned(16))) AlignedEnum { Value = 1 };

struct Outer7 {
    long long before;
    enum AlignedEnum inner;
};
