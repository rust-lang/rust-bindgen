// bindgen-flags: --remove-alias "int.*"

typedef long long int64;
typedef int int32;
typedef int32 i32;

struct int32_ {
    int32 inner;
};

int64 foo();
int32 bar();
i32 baz();
struct int32_ qux();
