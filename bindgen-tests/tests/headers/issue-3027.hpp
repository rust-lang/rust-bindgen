// bindgen-flags: --enable-cxx-namespaces

namespace regression {
    template<unsigned N> class A { char c[N]; };
    class C { A<3> a; };
}
