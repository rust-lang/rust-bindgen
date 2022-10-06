// bindgen-flags: -- -std=c++11

class Derived;
class Base {
public:
  virtual Derived* AsDerived();
};

class Derived final : public Base {
  virtual Derived* AsDerived() override;
};
