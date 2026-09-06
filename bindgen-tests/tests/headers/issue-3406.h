struct __attribute__((aligned(16))) Inner {
    char byte;
};

struct Outer {
    int before;
    struct Inner inner;
};
