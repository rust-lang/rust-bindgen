// bindgen-flags: --field-attr "Point::x=cfg(test)" --field-attr "Point::y=allow(dead_code)" --field-attr "Data::i=allow(dead_code)" --field-attr "Handle::0=cfg(test)" --new-type-alias "Handle"

struct Point {
    int x;
    int y;
};

union Data {
    int i;
    float f;
};

typedef int Handle;
