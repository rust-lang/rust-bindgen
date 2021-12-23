// A few tests for enum-related issues that should be tested with all the enum
// representations.

struct foo {
  enum {
    FOO_A,
    FOO_B,
  } member;
};

enum Foo {
    Bar = 0,
    Qux
};

enum Neg {
    MinusOne = -1,
    One = 1,
};

/** <div rustbindgen nodebug></div> */
enum NoDebug {
    NoDebug1,
    NoDebug2,
};

/** <div rustbindgen derive="Debug"></div> */
enum Debug {
    Debug1,
    Debug2,
};