// bindgen-flags: --with-derive-partialeq --impl-partialeq --use-core --raw-line "extern crate core;"

struct C {
    int large_array[420];
};
