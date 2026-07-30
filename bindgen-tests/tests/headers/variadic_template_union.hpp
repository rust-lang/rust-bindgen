// bindgen-flags: -- -std=c++14

namespace test {
template <class _Arg0, class... _Args>
union __union {
  _Arg0 __arg;
  __union<_Args...> __u;
};
}
