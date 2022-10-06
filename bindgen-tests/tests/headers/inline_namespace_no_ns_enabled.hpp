// bindgen-flags: -- -std=c++11

namespace std {
inline namespace __cxx11 {

template<typename CharT>
class basic_string {
  struct Alloc_hider {
    void* storage;
  } hider;
  unsigned long length;
  struct {
    CharT inline_storage[4];
  };
};

}
}
