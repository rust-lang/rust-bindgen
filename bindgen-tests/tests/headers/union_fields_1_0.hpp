// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq --with-derive-eq

typedef union {
  int mInt;
  float mFloat;
  void* mPointer;
} nsStyleUnion;
