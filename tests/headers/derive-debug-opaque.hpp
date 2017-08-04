// bindgen-flags: --opaque-type "Opaque" --force-derive-debug

class Opaque {
  int i;
  int not_debug[40];
};

class OpaqueUser {
  Opaque opaque;
};
