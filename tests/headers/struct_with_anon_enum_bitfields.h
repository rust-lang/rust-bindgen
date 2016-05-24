enum test {
    TEST_OPTION_1,
    TEST_OPTION_2,
    TEST_OPTION_3
};

struct foo {
    enum {
        FOO_OPTION_1,
        FOO_OPTION_2,
        FOO_OPTION_3
    } bar : 4;
    enum test baz : 4;
};
