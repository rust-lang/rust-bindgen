class A {
    unsigned c;
    struct Segment { int begin, end; };
    union {
        int f;
    } named_union;
    union {
        int d;
    };
};

class B {
    unsigned d;
    struct Segment { int begin, end; };
};
