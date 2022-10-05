// bindgen-flags: --rust-target 1.27 --enable-function-attribute-detection

class Foo {
public:
    __attribute__((warn_unused_result))
    int foo(int);
};

__attribute__((warn_unused_result))
int foo(int);
