// bindgen-flags: --anon-fields-prefix "u"

union color {
  struct {
    unsigned char r;
    unsigned char g;
    unsigned char b;
  };
  struct {
    unsigned char y;
    unsigned char u;
    unsigned char v;
  };
  unsigned char v3[3];
};
