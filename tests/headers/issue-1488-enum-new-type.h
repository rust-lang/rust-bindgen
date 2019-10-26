// bindgen-flags: --default-alias-style=new_type --constified-enum "Foo" --constified-enum-module "Bar" --rustified-enum "Qux" --new-type-alias-deref "BazAlias"

enum Foo {
    A,
    B
};

typedef enum Foo FooAlias;

enum Bar {
    C,
    D
};

typedef enum Bar BarAlias;

enum Qux {
    E,
    F
};

typedef enum Qux QuxAlias;

enum Baz {
    G,
    H
};

typedef enum Baz BazAlias;