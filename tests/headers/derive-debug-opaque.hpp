// bindgen-flags: --opaque-type "Opaque" --impl-debug

class Opaque {
  int i;
  int not_debug[40];
};

class OpaqueUser {
  Opaque opaque;
};
