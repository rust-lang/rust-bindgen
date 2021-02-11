// bindgen-flags: --generate functions --allowlist-function func --raw-line "#[repr(C)] pub struct nsTArray<T> { pub hdr: *const T }"

template<typename T>
class nsTArray {
  T* mHeader;
};

extern "C" nsTArray<int>* func();
