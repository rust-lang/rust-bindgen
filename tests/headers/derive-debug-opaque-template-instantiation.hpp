// bindgen-flags: --impl-debug

template<typename T, int N>
class Opaque {
  T array[N];
};

class Instance {
  Opaque<int, 50> val;
};
