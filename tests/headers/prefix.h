int test_var;
int test_fn();
struct test_struct {
    int i;
};
enum test_enum {
    TEST_A,
    TEST_B,
};
union test_union {
    struct test_struct s;
    enum test_enum e;
};

union test_union test_fn2();
struct test_struct test_fn3();
enum test_enum test_fn4();
