// bindgen-flags: --enable-cxx-namespaces --allowlist-type=std::string -- -std=c++11

namespace std {
  inline namespace bar {
    using string = const char*;
  };
};
