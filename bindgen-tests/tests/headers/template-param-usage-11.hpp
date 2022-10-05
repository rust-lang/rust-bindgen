// bindgen-flags: -- -std=c++14

template <typename T>
class DoesNotUseT {
    static T but_static_member_does;
};
