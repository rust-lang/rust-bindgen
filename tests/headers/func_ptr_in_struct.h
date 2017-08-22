// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
enum baz;

struct Foo {
    enum baz (*bar) (int x, int y);
};
