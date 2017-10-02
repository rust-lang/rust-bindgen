// bindgen-flags: --with-derive-partialeq --impl-partialeq

/// Deriving PartialEq for rust unions is not supported.
union ShouldNotDerivePartialEq {
    char a;
    int b;
};
