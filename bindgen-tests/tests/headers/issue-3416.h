// bindgen-flags: --rust-target 1.75 --rust-edition 2021 -- -std=gnu11

struct __attribute__((packed)) S {
    unsigned long long p : 2;
    unsigned long long x : 63;
};
