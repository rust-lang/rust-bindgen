// bindgen-flags: --with-derive-hash
//
enum baz;

struct Foo {
    enum baz (*bar) (int x, int y);
};
