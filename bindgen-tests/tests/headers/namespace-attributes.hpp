// bindgen-flags: --enable-cxx-namespaces -- -std=c++17

int take(int value);

namespace [[deprecated]] {
    int helper(int value);
}

namespace [[deprecated]] Named {
    int named(int value);
}
