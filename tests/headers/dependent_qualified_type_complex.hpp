struct InnerType {
    typedef int related_type;
    int foo;
    int foo2;
};

template <typename ContainedType> class Container {
public:
    typedef typename ContainedType::related_type content_ty;
    content_ty contents_;
};

typedef Container<InnerType> Concrete;

struct LaterContainingType {
    Concrete outer_contents;
};