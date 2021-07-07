// bindgen-flags: --impl-debug --rust-target 1.40

class Nice {
  typedef void (*Function) (int data);
  Function pointer;
  int large_array[34];
};
