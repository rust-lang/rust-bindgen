// bindgen-flags: -- -std=c++14

template <typename T, unsigned int Capacity>
struct Array {
    T elements[Capacity];
};

template <typename T>
using Array16 = Array<T, 16>;

using ArrayInt4 = Array<int, 4>;

struct UsesArray {
    Array16<char>  array_char_16;
    Array<bool, 8> array_bool_8;
    ArrayInt4      array_int_4;
};
