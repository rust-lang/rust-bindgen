// bindgen-flags: --with-derive-hash
template<typename T, typename ...Args>
struct Proxy {
  typedef void (*foo)(T* bar);
};
