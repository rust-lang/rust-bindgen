// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq

typedef union {
  int mInt;
  float mFloat;
  void* mPointer;
} nsStyleUnion;
