// bindgen-flags: --impl-debug

// This type is opaque because the second template parameter
// is a non-type template parameter
template<typename T, int N>
class Opaque {
  T array[N];
};

class Instance {
  Opaque<int, 50> val;
};
