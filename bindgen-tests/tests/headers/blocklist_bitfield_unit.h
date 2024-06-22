// bindgen-flags: --blocklist-type "__BindgenBitfieldUnit" --raw-line '#[path = "./struct_with_bitfields.rs"] mod bitfields;' --raw-line 'use bitfields::*;'
struct C {
  unsigned char x;
  unsigned b1 : 1;
  unsigned b2 : 1;
  unsigned baz;
};
