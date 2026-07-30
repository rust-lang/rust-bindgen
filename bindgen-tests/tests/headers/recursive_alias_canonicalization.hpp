// bindgen-flags: -- -std=c++14

namespace test {
union RecursiveUnion;
typedef RecursiveUnion AliasUnion;

union RecursiveUnion {
    int x;
    AliasUnion* self_ptr;
};
}
