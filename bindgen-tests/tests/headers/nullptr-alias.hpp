// bindgen-flags: --enable-cxx-namespaces -- -std=c++11

typedef decltype(nullptr) nullptr_t;
namespace nested {
    typedef decltype(nullptr) nullptr_t;
    nullptr_t identity(nullptr_t value);
}
nullptr_t identity(nullptr_t value);
