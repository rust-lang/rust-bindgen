
namespace cv {
  class Foo {
  public:
      ~Foo();
  };

  Foo::~Foo() {}

  class Bar {
  public:
      ~Bar();
  };

  inline
  Bar::~Bar() {}

}
