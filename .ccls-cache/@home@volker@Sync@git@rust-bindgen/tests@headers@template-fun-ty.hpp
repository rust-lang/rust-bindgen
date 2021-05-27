// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
template <typename T>
class Foo
{
public:
  typedef T (FunctionPtr)();
};

template<typename T>
class RefPtr {
public:
  template<typename R, typename... Args>
  class Proxy {
  public:
    typedef R (T::*member_function)(Args...);
  };
};

template<typename T>
using Returner = T(*)();
