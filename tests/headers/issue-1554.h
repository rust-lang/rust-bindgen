// bindgen-flags: --default-enum-style rust_non_exhaustive --raw-line '#![cfg(feature = "nightly")]' --raw-line '#![feature(non_exhaustive)]'

enum Planet {
    earth,
    mars
};
