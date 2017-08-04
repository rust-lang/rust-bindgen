// bindgen-flags: --opaque-type "Opaque" --force-derive-debug

class C {
  bool a: 1;
  bool b: 7;
  int large_array[50];
};
