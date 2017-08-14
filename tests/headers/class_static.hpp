// bindgen-flags: --with-derive-hash --with-derive-partialeq
class MyClass {
public:
    static const int* example;
    static const int* example_check_no_collision;
};

static const int* example_check_no_collision;
