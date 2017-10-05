// bindgen-flags: --with-derive-partialeq --impl-partialeq --impl-debug

/// Because this struct have array larger than 32 items 
/// and --with-derive-partialeq --impl-partialeq --impl-debug is provided, 
/// this struct should manually implement `Debug` and `PartialEq`.
struct Foo {
    int large[33];
    char type_ : 3;
    unsigned : 8;
    char type();
    void set_type_(char c);
    void set_type(char c);
};
