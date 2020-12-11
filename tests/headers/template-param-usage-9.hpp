// bindgen-flags: -- -std=c++14

template <typename T, typename U>
class DoesNotUse {
public:
    using Aliased = T;
    typedef U Typedefed;

    class IndirectUsage {
        Aliased member;
        Typedefed another;
    };
};
