// bindgen-flags: --blacklist-type RefPtr_Proxy_member_function

template<typename T>
class RefPtr {
public:
  template<typename R, typename... Args>
  class Proxy {
    typedef R (T::*member_function)(Args...);
  };
};
