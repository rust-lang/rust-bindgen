// bindgen-flags: --rustified-enum ".*"

enum {
  FOO_BAR,
  FOO_BAZ,
};

class Foo {
public:
  enum { FOO_BAR = 10 };
};
