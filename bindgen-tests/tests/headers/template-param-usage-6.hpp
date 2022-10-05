// bindgen-flags: -- -std=c++14

template <typename T>
class DoesNotUseTemplateParameter {
    using ButAliasDoesUseIt = T;

    int x;
};
