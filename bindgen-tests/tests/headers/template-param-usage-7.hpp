// bindgen-flags: -- -std=c++14

template <typename T, typename U, typename V>
class DoesNotUseU {
    T t;
    V v;
};

// The bool should go away becuase U is not used.
using Alias = DoesNotUseU<int, bool, char>;
