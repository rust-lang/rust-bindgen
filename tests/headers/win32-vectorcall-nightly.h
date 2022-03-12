// bindgen-flags: --rust-target nightly --raw-line '#![cfg(feature = "nightly")]' --raw-line '#![feature(abi_vectorcall)]' -- --target=x86_64-pc-windows-msvc

int __vectorcall test_vectorcall(int a, int b);
