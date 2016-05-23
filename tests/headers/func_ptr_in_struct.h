enum baz {
    TEST
};

struct Foo {
    enum baz (*bar) (int x, int y);
};
