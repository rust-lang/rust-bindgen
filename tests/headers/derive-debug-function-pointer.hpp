// bindgen-flags: --impl-debug

class Nice {
  typedef void (*Function) (int data);
  Function pointer;
  int large_array[34];
};
