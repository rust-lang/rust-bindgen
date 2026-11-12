// bindgen-flags: --allowlist-type OuterStruct --no-recursive-allowlist --raw-line "pub struct UnparsedTemplate(pub u32);"

template <typename T>
struct UnparsedTemplate {
    T val;
    template <typename U> void unparsed_method(U u);
};

struct OuterStruct {
    UnparsedTemplate<int> field;
};
