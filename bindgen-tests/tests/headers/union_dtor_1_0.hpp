// bindgen-flags: --rust-target 1.0

union UnionWithDtor {
  ~UnionWithDtor();
  int mFoo;
  void* mBar;
};
