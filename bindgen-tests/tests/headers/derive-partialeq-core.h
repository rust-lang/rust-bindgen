// bindgen-flags: --with-derive-partialeq --impl-partialeq --use-core --raw-line "extern crate core;" --rust-target 1.40

struct C {
    int large_array[420];
};
