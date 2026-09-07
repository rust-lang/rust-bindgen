// bindgen-flags: -- -std=c++23
// bindgen-min-clang-version: 18

struct S {
    int value(this S self, int y);
    int reference([[maybe_unused]] this S& self, int y);
    int regular(int y);
};
