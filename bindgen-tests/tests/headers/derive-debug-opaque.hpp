// bindgen-flags: --opaque-type "Opaque" --impl-debug --rust-target 1.40

class Opaque {
  int i;
  int not_debug[40];
};

class OpaqueUser {
  Opaque opaque;
};
