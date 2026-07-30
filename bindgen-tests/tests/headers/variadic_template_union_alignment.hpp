// bindgen-flags: --raw-line '#[test] fn test_mixed_union_pointer_size() { assert_eq!(::std::mem::size_of::<test_MixedCharPointer>(), 8); }' --opaque-type '.*MixedUnionValue.*' -- -std=c++14

namespace test {

template <class T, class... Args>
union MixedUnionPointer {
    T val;
    MixedUnionPointer<Args...>* ptr;
};

template <class T>
union MixedUnionPointer<T> {
    T val;
};

template <class T, class... Args>
union MixedUnionValue {
    T val;
    MixedUnionValue<Args...> next;
};

template <class T>
union MixedUnionValue<T> {
    T val;
};

using MixedCharPointer = MixedUnionPointer<char, int>;
using MixedCharDouble = MixedUnionValue<char, double>;

static_assert(sizeof(MixedCharPointer) == 8, "Expected sizeof(MixedCharPointer) == 8 in C++");
static_assert(sizeof(MixedCharDouble) == 8, "Expected sizeof(MixedCharDouble) == 8 in C++");

}
