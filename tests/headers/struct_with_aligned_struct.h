typedef signed short      int16_t;
typedef signed int        int32_t;
typedef signed long long  int64_t;

struct foo {
    int32_t x;
    int64_t y;
    int16_t z;
} __attribute__ ((aligned (64)));

struct bar {
    int32_t a;
    int64_t b;
    struct foo foo;
} __attribute__ ((aligned (64)));
