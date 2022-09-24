// bindgen-flags: -- -std=c++14

template <typename T>
class IndirectlyUsesTemplateParameter {
    using Aliased = T;

    Aliased aliased;
};
