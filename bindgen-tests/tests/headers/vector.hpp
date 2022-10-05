struct foo {
  __attribute__((__vector_size__(1 * sizeof(long long)))) long long mMember;
};

typedef float __m128 __attribute__ ((__vector_size__ (16)));
typedef double __m128d __attribute__((__vector_size__(16)));
typedef long long __m128i __attribute__((__vector_size__(16)));

__m128 foo(__m128i, __m128d);
