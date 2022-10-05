class PureVirtualIFace {
public:
    virtual void Foo() = 0;
    virtual void Bar(unsigned int) = 0;
};

class AnotherInterface {
public:
    virtual void Baz() = 0;
};

class Implementation : public PureVirtualIFace {
public:
    void Foo() override {}
    void Bar(unsigned int) override {}
};

class DoubleImpl : public PureVirtualIFace, public AnotherInterface {
public:
    void Foo() override {}
    void Bar(unsigned int) override {}

    void Baz() override {}
};
