// bindgen-flags: --with-derive-hash --with-derive-partialeq
//
enum baz;

struct Foo {
    enum baz (*bar) (int x, int y);
};
