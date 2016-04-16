template<typename T, typename U> class Foo {
    T m_member;
    T* m_member_ptr;
    T m_member_arr[1];
};

void bar(Foo<int, int> foo);

template<typename T>
class D {
    typedef Foo<int, int> MyFoo;

    MyFoo m_foo;

    template<typename Z>
    class U {
        MyFoo m_nested_foo;
        Z m_baz;
    };
};

template<typename T>
class Rooted {
    T* prev;
    T* next;
    T ptr;
};

class RootedContainer {
    Rooted<void*> root;
};
