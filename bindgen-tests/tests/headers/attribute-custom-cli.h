// bindgen-flags: --default-enum-style rust --default-non-copy-union-style manually_drop --no-default=".*" --no-hash=".*" --no-partialeq=".*" --no-debug=".*" --no-copy=".*" --with-attribute-custom="foo_[^e].*=#[doc(hidden)]" --with-attribute-custom-struct="foo.*=#[derive(Default)]" --with-attribute-custom-enum="foo.*=#[cfg_attr(test, derive(PartialOrd, Copy))]" --with-attribute-custom-union="foo.*=#[derive(Clone)],#[derive(Copy)]"
struct foo_struct {
    int inner;
};
enum foo_enum {
    inner = 0
};
union foo_union {
    int fst;
    float snd;
};
struct non_matching {
    int inner;
};
