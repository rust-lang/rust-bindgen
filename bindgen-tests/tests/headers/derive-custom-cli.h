// bindgen-flags: --default-enum-style rust --default-non-copy-union-style manually_drop --no-default=".*" --no-hash=".*" --no-partialeq=".*" --no-debug=".*" --no-copy=".*" --with-derive-custom="foo_[^e].*=Clone" --with-derive-custom-struct="foo.*=Default" --with-derive-custom-enum="foo.*=Copy" --with-derive-custom-union="foo.*=Copy"
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
