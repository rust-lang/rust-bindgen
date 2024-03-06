// bindgen-flags: --clang-macro-fallback

#define UINT32_C(c) c ## U

#define CONST UINT32_C(5)
#define OTHER_CONST UINT32_C(6)
#define LARGE_CONST UINT32_C(6 << 8)
