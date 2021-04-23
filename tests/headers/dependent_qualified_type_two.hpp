struct InnerTypeA {
    typedef int related_type;
    int foo;
    int foo2;
};

struct InnerTypeB {
    typedef char related_type;
    unsigned long bar;
};

template <typename ContainedType> class Container {
public:
    typedef typename ContainedType::related_type content_ty;
    content_ty contents_;
};

typedef Container<InnerTypeA> Concrete;

struct LaterContainingType {
    Concrete outer_contents_a;
    Container<InnerTypeB> outer_contents_b;
};
