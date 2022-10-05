// bindgen-flags: --generate-inline-functions -- -std=c++11

class A {
public:
      // Deleted function should not get a binding.
      void deleted() = delete;

      // Inline functions should get bindings, whether they are defined inline
      // (in the class) or out of line.
      inline void inline_definition() {}
      inline void out_of_line_definition();

      // TODO: This is an edge case that we get wrong: An inline function
      // without a definition in the same translation unit should still get a
      // binding. We currently can't distinguish this case from a deleted member
      // function because libclang doesn't provide a direct way to query for
      // deleted member functions. This seems acceptable, however, as an inline
      // function without a definition in the same translation unit is unlikely
      // to be useful in practice.
      inline void inline_without_definition();
};

void A::out_of_line_definition() {}

class B {
public:
     // Deleted copy constructor should not get a binding.
     B(B&) = delete;
};

class C {
public:
     // Defaulted copy constructor should get a binding.
     C(C&) = default;
};