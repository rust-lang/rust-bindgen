// bindgen-flags: --default-enum-style=consts --constified-enum-module=Neg

// A forward reference to an enum should work..
enum Foo;
enum Foo {
  A
};

// A reference to an enum after the definition should work.
enum Bar { B };
enum Bar;

// A forward reference to an enum in a function definition should work.
enum Baz f(void);
enum Baz { C };

// A forward reference to an enum with no later definition should result in
// a guess of the underlying type (i32).
enum Qux;
