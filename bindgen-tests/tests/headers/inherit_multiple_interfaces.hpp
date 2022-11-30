class A {
  virtual void Foo();

  int member;
};

class B {
  virtual void Bar();

  void* member2;
};

class C : public A, public B {
  float member3;
};
