// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --impl-partialeq

struct Foo {
    char big_array[33];
    char type_ : 3;
    char type();
    void set_type_(char c);
    void set_type(char c);
};
