// bindgen-flags: --remove-alias "int.*"

typedef long long int64;
typedef int int32;
typedef int32 i32;
struct int32_ {
    int32 inner;
};
typedef int32* int32_ptr;

int64 returns_int64();
int32 returns_int32();
i32 returns_i32();
struct int32_ returns_int32_();
int32_ptr returns_int32_ptr();

template<typename T>
using integer = T;
