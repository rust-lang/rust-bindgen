// bindgen-flags: --use-distinct-char16-t --raw-line '#[repr(transparent)] pub struct bindgen_cchar16_t(u16);' -- -x c++ -std=c++14

void receive_char16_t(char16_t input) {
}
