// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
class MyClass {
public:
    static const int* example;
    static const int* example_check_no_collision;
};

static const int* example_check_no_collision;
