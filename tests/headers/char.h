// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
typedef char Char;
typedef signed char SChar;
typedef unsigned char UChar;

struct Test {
  char ch;
  unsigned char u;
  signed char d;
  const char cch;
  const unsigned char cu;
  const signed char cd;

  Char Cch;
  UChar Cu;
  SChar Cd;
  const Char Ccch;
  const UChar Ccu;
  const SChar Ccd;
};
