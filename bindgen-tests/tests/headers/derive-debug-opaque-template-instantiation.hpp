// bindgen-flags: --impl-debug --rust-target 1.40

// This type is opaque because the second template parameter
// is a non-type template parameter
template<typename T, int N>
class Opaque {
  T array[N];
};

class Instance {
  Opaque<int, 50> val;
};
