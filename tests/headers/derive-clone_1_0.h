// bindgen-flags: --rust-target 1.0

/// Since builtin `Clone` impls were introduced in Rust 1.21 this struct 
/// should impl `Clone` "manually".
struct ShouldImplClone {
    int large[33];
};
