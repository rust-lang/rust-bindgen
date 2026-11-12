// bindgen-flags: --allowlist-type OuterArrayStruct --no-recursive-allowlist --raw-line "pub struct UnparsedArrayElem(pub u32);"

template <typename T>
struct UnparsedArrayElem {
    T val;
    template <typename U> void unparsed_method(U u);
};

struct OuterArrayStruct {
    UnparsedArrayElem<int> flex_array[0];
};
