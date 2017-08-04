// bindgen-flags: --force-derive-debug

class Nice {
  typedef void (*Function) (int data);
  Function pointer;
  int large_array[34];
};
