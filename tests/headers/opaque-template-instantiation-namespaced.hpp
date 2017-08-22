// bindgen-flags: --enable-cxx-namespaces --opaque-type 'zoidberg::Template<zoidberg::Bar>'  --with-derive-hash --with-derive-partialeq --with-derive-eq -- -std=c++14

namespace zoidberg {

template <typename T>
class Template {
    T member;
};

struct Foo {
    char c;
};

struct Bar {
    int i;
};

class ContainsInstantiation {
    Template<Foo> not_opaque;
};

class ContainsOpaqueInstantiation {
    // We should not generate a layout test for this instantiation, and it
    // should appear as an opaque blob of bytes in
    // `ContainsOpaqueInstantiation`'s type definition.
    Template<Bar> opaque;
};

}
