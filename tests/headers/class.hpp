class C {
    int a;
    // More than rust limits (32)
    char big_array[33];
    char zero_length_array[0];
    char incomplete_array[];
};

class WithDtor {
    int b;

    ~WithDtor() {}
};

union Union {
    float d;
    int i;
};

class WithUnion {
    Union data;
};

class RealAbstractionWithTonsOfMethods {
  void foo();
public:
  void bar() const;
  void bar();
  void bar(int foo);
  static void sta();
};
