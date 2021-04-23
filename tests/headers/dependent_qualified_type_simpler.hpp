struct InnerType {
    typedef int related_type;
    long int foo;
};

template <typename ContainedType> class Container {
public:
    typename ContainedType::related_type contents_;
};

typedef Container<InnerType> Concrete;
