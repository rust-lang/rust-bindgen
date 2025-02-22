void a();

namespace B {
    void c();

    namespace D {
        void e();
    }

    // We should not report empty namespaces
    namespace F {
    }

    namespace {
        void g();
    }

    inline namespace H {
        void i();
        namespace J {
            void k();
        }
    }

    struct L {
        struct M {

        };
    };
};