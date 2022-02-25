// bindgen-flags: --rustified-enum ".*" --raw-line '#![cfg(not(target_os="windows"))]'
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
