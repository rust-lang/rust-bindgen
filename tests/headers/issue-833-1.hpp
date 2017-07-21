// bindgen-flags: --generate functions --whitelist-function func --raw-line "#[repr(C)] pub struct nsTArray { pub hdr: *const () }"

template<typename T>
class nsTArray {
  static T* sFoo;
};

extern "C" nsTArray<int>* func();
