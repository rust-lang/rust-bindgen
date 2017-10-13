// bindgen-flags: -- -std=c++14

// Small test that we handle virtual tables correctly when deriving from a
// template instantiation. This wasn't previously handled at all. Note that when
// inheriting from a template parameter, the type that is instantiated might or
// might not have a virtual table, and we have no way of knowing. We don't
// handle that yet, so no test for it here.

/// This should have an explicit vtable.
template<class T>
class BaseWithVtable {
    T t;

    virtual void hello();
};

/// This should not have an explicit vtable.
class DerivedWithNoVirtualMethods : public BaseWithVtable<char*> {};

/// This should not have an explicit vtable.
class DerivedWithVirtualMethods : public BaseWithVtable<char*> {
    virtual void zoidberg();
};

/// This should not have any vtable.
template<class U>
class BaseWithoutVtable {
    U u;
};

/// This should have an explicit vtable.
class DerivedWithVtable : public BaseWithoutVtable<char*> {
    virtual void leela();
};

/// This should not have any vtable.
class DerivedWithoutVtable : public BaseWithoutVtable<char*> {};
