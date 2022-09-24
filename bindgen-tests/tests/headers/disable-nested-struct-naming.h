// bindgen-flags: --disable-nested-struct-naming

struct foo {
    struct bar1 {
        int x1;
        struct {
            int x2;
            struct {
                int x3;
                struct bar4 {
                    int x4;
                } b4;
            } b3;
        } b2;
    } b1;
};

struct {
    struct {
        struct baz {
            int x;
        } b;
    } anon2;
} anon1;
