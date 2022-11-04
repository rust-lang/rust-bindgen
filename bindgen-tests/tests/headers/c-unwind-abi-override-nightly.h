// bindgen-flags: --override-abi="foo|bar=C-unwind" --rust-target=nightly --raw-line '#![cfg(feature = "nightly")]' --raw-line '#![feature(abi_thiscall)]'

void foo();
void bar();
void baz();
