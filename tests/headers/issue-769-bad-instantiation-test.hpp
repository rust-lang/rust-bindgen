// bindgen-flags: --enable-cxx-namespaces --whitelist-type Rooted

template <typename T>
class Rooted {
    T member;
};

class AutoValueVector : Rooted<int> {
public:
    using Alias = int;
    using RootedAlias = Rooted<Alias>;
};
