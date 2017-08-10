// bindgen-flags: --with-derive-hash --with-derive-partialeq
template<typename T, typename ...Args>
struct Proxy {
  typedef void (*foo)(T* bar);
};
