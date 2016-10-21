
#define COMPLEX_TEST(ty_)     \
  struct Test##ty_ {          \
    ty_ _Complex mMember;     \
                              \
  };                          \
  struct Test##ty_##Ptr {     \
    ty_ _Complex* mMember;    \
  };

COMPLEX_TEST(double)
COMPLEX_TEST(float)
COMPLEX_TEST(int)
