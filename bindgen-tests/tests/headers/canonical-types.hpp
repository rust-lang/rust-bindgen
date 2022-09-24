// bindgen-flags: -- -std=c++14

// Issue #2078. To pick up the definition of `ClassCInnerA`,
// `ty.canonical_type().declaration().definition()` is needed.

template<class T>
class ClassA {
public:
    class ClassAInner {
    public:
        T *x;
    };
};

template<class D, class I>
class ClassB {
public:
    void foo() {
        ((D *)0)->quux();
    }
};

template<typename T>
class ClassC {
    struct ClassCInnerA;

    struct ClassCInnerB {
        ClassCInnerA *cache;
    };
    static_assert(sizeof(ClassCInnerB) > 0, "");

    struct ClassCInnerA {
        ClassCInnerB *member;
    };

public:
    class ClassCInnerCRTP : public ClassB<ClassCInnerCRTP, typename ClassA<ClassCInnerA>::ClassAInner> {
    public:
        void quux() {
            ((typename ClassA<ClassCInnerA>::ClassAInner *)0)->x->member;
        }
    };
};

class ClassD : public ClassB<ClassD, ClassC<int>::ClassCInnerCRTP> {
public:
    void bar() {
        ((ClassC<int>::ClassCInnerCRTP *)0)->foo();
    }
};

