// bindgen-flags: --rust-target 1.33

class TestOverload {
  // This one shouldn't be generated.
  TestOverload();
public:
  /// Calling this should use `mem::unintialized()` and not `MaybeUninit()` as only rust 1.36 includes that.
  TestOverload(int);
  /// Calling this should use `mem::unintialized()` and not `MaybeUninit()` as only rust 1.36 includes that.
  TestOverload(double);
};

class TestPublicNoArgs {
public:
  TestPublicNoArgs();
};
