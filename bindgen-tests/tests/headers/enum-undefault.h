// bindgen-flags: --default-enum-style=rust --constified-enum=Neg

enum Foo {
    Bar = 0,
    Qux
};

enum Neg {
    MinusOne = -1,
    One = 1,
};
