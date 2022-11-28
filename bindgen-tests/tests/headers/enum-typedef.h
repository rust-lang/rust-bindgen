typedef short int16_t;

// `cbindgen` emits this C idiom as the translation of:
//
//     #[repr(i16)]
//     pub enum Enum {
//         Variant,
//     }
enum Enum {
  Variant,
};
typedef int16_t Enum;

// C is also fine with the typedef coming before the enum.
typedef int16_t TypedefFirst;
enum TypedefFirst {
  Variant2,
};
