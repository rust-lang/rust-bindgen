// bindgen-flags: -- -std=c++14

template <typename T, typename U>
class IndirectUsage {
    typedef T Typedefed;
    using Aliased = U;

    Typedefed member1;
    Aliased member2;
};
