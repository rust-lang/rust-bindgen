// bindgen-flags: --with-derive-partialeq --impl-partialeq

class Base {
    int large[33];
};

class ShouldDerivePartialEq: Base {
};
