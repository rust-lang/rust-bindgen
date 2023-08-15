// bindgen-flags: --allowlist-item 'Foo.*'

struct Foo {
  int field;
};

struct Foo FooNew(int value);

#define FooDefault 0 

struct Bar {
  int field;
};

struct Foo BarNew(int value);

#define BarDefault 0  
