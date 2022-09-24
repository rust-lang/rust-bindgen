// bindgen-flags: -- -std=c++14

template <typename T, typename U, typename NeverUsed>
class DoublyIndirectUsage {
    using Aliased = T;
    typedef U Typedefed;

    class IndirectUsage {
        Aliased member;
        Typedefed another;
    };

    IndirectUsage doubly_indirect;
};
