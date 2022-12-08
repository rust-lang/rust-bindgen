// bindgen-flags: --sort-semantically --enable-cxx-namespaces -- --target=x86_64-unknown-linux

class Foo {
public:
    int foo;
    int get_foo();
};

class Bar {
public:
    int bar;
    int get_bar();
};