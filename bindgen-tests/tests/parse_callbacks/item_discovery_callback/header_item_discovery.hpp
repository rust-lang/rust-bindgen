// Methods

class SomeClass {
public:
    SomeClass() = delete;
    SomeClass(const SomeClass&) = default;
    SomeClass(SomeClass&&);
    void named_method();
    virtual void virtual_method();
    virtual void pure_virtual_method() = 0;
private:
    void private_method();
protected:
    void protected_method();

};