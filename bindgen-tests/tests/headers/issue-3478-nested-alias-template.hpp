template <typename T>
using Wrapper = T*;

template <typename T>
using Id = T;

template <typename T>
struct Holder {
    T value;
};

template <typename T>
using HolderOf = Holder<T>;

struct Nested {
    Id<Id<int>> a;
    Wrapper<Wrapper<char>> b;
    Id<Wrapper<Id<int>>> c;
    HolderOf<HolderOf<int>> d;
};

template <typename T>
struct Generic {
    Id<Id<T>> x;
};

struct Outer {
    template <typename T>
    using Member = T;

    Member<Member<int>> m;
};

typedef struct {
    Wrapper<Wrapper<Wrapper<char>>> chars;
    int size;
} Anonymous;
