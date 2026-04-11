// bindgen-flags: --rust-target=1.77

// Regression test for #3295: clang exposes the anonymous struct declared
// inside `__alignof__(...)` of an aligned attribute as a child of the
// surrounding struct. Bindgen used to treat that child as an extra
// anonymous member, doubling the surrounding struct's layout and
// producing failing size assertions in the generated bindings.

typedef struct base_t {
    struct {
        int aaa;
    } __attribute__((aligned(
        __alignof__(
            struct {int aaa;}
        )
    )));
} base_t;
