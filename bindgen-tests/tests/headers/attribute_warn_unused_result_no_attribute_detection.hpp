// bindgen-flags: \-\-rust-target=1.33

class Foo {
public:
    __attribute__((warn_unused_result))
    int foo(int);
};

__attribute__((warn_unused_result))
int foo(int);
