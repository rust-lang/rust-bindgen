// bindgen-flags: --no-derive-copy --no-derive-default --no-derive-debug --with-custom-derive Clone,Copy/ShouldDerive.*

struct ShouldDeriveStruct {
    char a;
    int b;
    struct {
        int c;
    } nested;
};

union ShouldDeriveUnion {
    char a;
    int b;
};

struct ShouldNotDeriveStruct {
    char a;
    int b;
};
