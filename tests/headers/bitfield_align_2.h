// bindgen-flags: --rustified-enum .*
enum MyEnum {
    ONE,
    TWO,
    THREE,
    FOUR
};

struct TaggedPtr {
    enum MyEnum tag : 2;
    long   ptr : 62;
};
