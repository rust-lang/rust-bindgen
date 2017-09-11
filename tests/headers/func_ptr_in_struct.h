// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --rustified-enum .*
//
enum baz;

struct Foo {
    enum baz (*bar) (int x, int y);
};
