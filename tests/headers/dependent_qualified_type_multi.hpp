struct InnerType {
    typedef int related_type;
    long int foo;
};
struct InnerType2 {
    typedef float related_type;
    long int foo;
};

template <typename ContainedType> class Container {
public:
    typename ContainedType::related_type contents_;
};

typedef Container<InnerType> Concrete;
typedef Container<InnerType2> Concrete2;
