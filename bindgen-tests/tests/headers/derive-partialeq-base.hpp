// bindgen-flags: --with-derive-partialeq --impl-partialeq --rust-target 1.40

class Base {
    int large[33];
};

class ShouldDerivePartialEq: Base {
};
