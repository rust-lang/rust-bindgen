// bindgen-flags: --with-derive-hash --with-derive-partialeq --enable-cxx-namespaces

namespace whatever {
    typedef struct {
        int foo;
    } typedef_struct;

    typedef enum {
        BAR=1
    } typedef_enum;
}

namespace {
    typedef struct {
        int foo;
    } typedef_struct;

    typedef enum {
        BAR=1
    } typedef_enum;
}
