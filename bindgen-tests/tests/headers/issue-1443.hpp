struct Foo;

struct Bar {
  const Foo& f;
  unsigned m;
};

struct Baz {
  Foo& f;
  unsigned m;
};

struct Tar {
  const Foo&& f;
  unsigned m;
};

struct Taz {
  Foo&& f;
  unsigned m;
};
