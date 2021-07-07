// bindgen-flags: --no-default "NoDefault" --rust-target 1.40

template<typename T>
class Generic {
  T t[40];
};

template<typename T>
class NoDefault {
  T t[40];
};
