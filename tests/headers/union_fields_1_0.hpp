// bindgen-flags: --rust-target 1.0 --with-derive-hash

typedef union {
  int mInt;
  float mFloat;
  void* mPointer;
} nsStyleUnion;
