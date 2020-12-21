struct Foo {
  unsigned int a;
  double b;
};

typedef struct Foo Foo;

void takes_foo(Foo);
Foo gives_foo(void);
