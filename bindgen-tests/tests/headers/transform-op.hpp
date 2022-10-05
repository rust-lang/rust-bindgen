// bindgen-flags: --rust-target 1.0 -- -std=c++11

typedef unsigned char uint8_t;
typedef int int32_t;

template<typename T>
struct StylePoint {
  T x;
  T y;
};

template<typename T>
union StyleFoo {
  enum class Tag : uint8_t {
    Foo,
    Bar,
    Baz,
    Bazz,
  };

  struct Foo_Body {
    Tag tag;
    int32_t x;
    StylePoint<T> y;
    StylePoint<float> z;
  };

  struct Bar_Body {
    Tag tag;
    T _0;
  };

  struct Baz_Body {
    Tag tag;
    StylePoint<T> _0;
  };

  struct {
    Tag tag;
  };
  Foo_Body foo;
  Bar_Body bar;
  Baz_Body baz;
};

template<typename T>
struct StyleBar {
  enum class Tag {
    Bar1,
    Bar2,
    Bar3,
    Bar4,
  };

  struct StyleBar1_Body {
    int32_t x;
    StylePoint<T> y;
    StylePoint<float> z;
  };

  struct StyleBar2_Body {
    T _0;
  };

  struct StyleBar3_Body {
    StylePoint<T> _0;
  };

  Tag tag;
  union {
    StyleBar1_Body bar1;
    StyleBar2_Body bar2;
    StyleBar3_Body bar3;
  };
};
