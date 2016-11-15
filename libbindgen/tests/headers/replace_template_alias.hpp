// bindgen-flags: -- --std=c++14

namespace JS {
namespace detail {

/// Notice how this doesn't use T.
template <typename T>
using MaybeWrapped = int;

}

template <typename T>
class Rooted {
    detail::MaybeWrapped<T> ptr;
};

}

/// But the replacement type does use T!
///
/// <div rustbindgen replaces="JS_detail_MaybeWrapped" />
template <typename T>
using replaces_MaybeWrapped = T;
