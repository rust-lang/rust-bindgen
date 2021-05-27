// bindgen-flags: -- -std=c++14

template <typename T>
class DoesNotUseTemplateParameter {
public:
    using ButAliasDoesUseIt = T;

    int x;
};
