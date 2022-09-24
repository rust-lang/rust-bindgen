// bindgen-flags: --c-naming

typedef const struct a {
    int a;
} *a;

union b {
    int a;
    int b;
};
typedef union b b;

enum c {
    A,
};

void takes_a(a arg) {}
void takes_b(b arg) {}
void takes_c(enum c arg) {}
