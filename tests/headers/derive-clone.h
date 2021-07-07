// bindgen-flags: --rust-target 1.40
//

/// This struct should derive `Clone`.
struct ShouldDeriveClone {
    int large[33];
};
