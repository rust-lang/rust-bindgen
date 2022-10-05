// bindgen-flags: -- -std=c++14

template <typename T, typename U>
class DoesNotUse {
    using Aliased = T;
    typedef U Typedefed;

    class IndirectUsage {
        Aliased member;
        Typedefed another;
    };
};
