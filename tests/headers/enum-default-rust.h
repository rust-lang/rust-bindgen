// bindgen-flags: --default-enum-style=rust --constified-enum-module=Neg

enum Foo {
    Bar = 0,
    Qux
};

enum Neg {
    MinusOne = -1,
    One = 1,
};
