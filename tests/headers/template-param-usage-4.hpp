// bindgen-flags: -- -std=c++14

template <typename T>
class UsesTemplateParameter {
    T t;

    template <typename U>
    class DoesNotUseTemplateParameters {
        int x;
    };
};
