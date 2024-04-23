
class TestOverload {
  // This one shouldn't be generated.
  TestOverload();
public:
  TestOverload(int);
  TestOverload(double);
};

class TestPublicNoArgs {
public:
  TestPublicNoArgs();
};
