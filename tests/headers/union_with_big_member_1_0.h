// bindgen-flags: --rust-target 1.0

union WithBigArray {
  int a;
  int b[33];
};

union WithBigArray2 {
  int a;
  char b[33];
};

union WithBigMember {
  int a;
  union WithBigArray b;
};
