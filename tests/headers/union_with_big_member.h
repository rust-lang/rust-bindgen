// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
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
