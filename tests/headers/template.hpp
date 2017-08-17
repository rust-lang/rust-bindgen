// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq -- -std=c++11
//
template<typename T, typename U> class Foo {
    T m_member;
    T* m_member_ptr;
    T m_member_arr[1];
};

template<typename T> class B {
    T m_member { 0 };
};

void bar(Foo<int, int> foo);

namespace mozilla {
class Foo;
};

struct C {
    B<unsigned int> mB;
    B<const int*> mBConstPtr;
    B<const mozilla::Foo*> mBConstStructPtr;
    B<const mozilla::Foo*[1]> mBConstStructPtrArray;
    B<const int> mBConst;
    B<volatile int> mBVolatile;
    B<const bool> mBConstBool;
    B<const char16_t> mBConstChar;
    B<int[1]> mBArray;
    B<int*[1]> mBPtrArray;
    B<int(*)[1]> mBArrayPtr;
    B<int&> mBRef;
    B<const int&> mBConstRef;
    B<int*&> mPtrRef;
    B<int(&)[1]> mArrayRef;
    // clang 3.x ignores const in this case, so they generate different
    // result than clang 4.0.
    // B<const int[1]> mBConstArray;
};

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
    Rooted<void*>* next;
    T ptr;
};

class RootedContainer {
    Rooted<void*> root;
};

template<typename T>
class WithDtor;

typedef WithDtor<int> WithDtorIntFwd;

template<typename T>
class WithDtor {
    T member;
    ~WithDtor() {}
};

class PODButContainsDtor {
    WithDtorIntFwd member;
};


/** <div rustbindgen opaque> */
template<typename T>
class Opaque {
    T member;
};

class POD {
    Opaque<int> opaque_member;
};

/**
 * <div rustbindgen replaces="NestedReplaced"></div>
 */
template<typename T>
class Nested {
    T* buff;
};

template<typename T, typename U>
class NestedBase {
    T* buff;
};

template<typename T>
class NestedReplaced: public NestedBase<T, int> {
};

template<typename T>
class Incomplete;

template<typename T>
class NestedContainer {
    T c;
private:
    NestedReplaced<T> nested;
    Incomplete<T> inc;
};

template<typename T>
class Incomplete {
    T d;
};

class Untemplated {};

template<typename T>
class Templated {
    Untemplated m_untemplated;
};

/**
 * If the replacement doesn't happen at the parse level the container would be
 * copy and the replacement wouldn't, so this wouldn't compile.
 *
 * <div rustbindgen replaces="ReplacedWithoutDestructor"></div>
 */
template<typename T>
class ReplacedWithDestructor {
    T* buff;
    ~ReplacedWithDestructor() {};
};

template<typename T>
class ReplacedWithoutDestructor {
    T* buff;
};

template<typename T>
class ReplacedWithoutDestructorFwd;

template<typename T>
class ShouldNotBeCopiable {
    ReplacedWithoutDestructor<T> m_member;
};

template<typename U>
class ShouldNotBeCopiableAsWell {
    ReplacedWithoutDestructorFwd<U> m_member;
};

/**
 * If the replacement doesn't happen at the parse level the container would be
 * copy and the replacement wouldn't, so this wouldn't compile.
 *
 * <div rustbindgen replaces="ReplacedWithoutDestructorFwd"></div>
 */
template<typename T>
class ReplacedWithDestructorDeclaredAfter {
    T* buff;
    ~ReplacedWithDestructorDeclaredAfter() {};
};
