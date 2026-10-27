// bindgen-flags: --clang-macro-fallback '^DO_FALLBACK_\S+$' --clang-macro-fallback ALSO_THIS_ONE

#ifndef CLANG_MACRO_FALLBACK_LIMIT_H
#define CLANG_MACRO_FALLBACK_LIMIT_H

#define UINT32_C(c) c ## U

#define DO_FALLBACK_A UINT32_C(5)
#define DO_FALLBACK_B UINT32_C(6 << 8)

#define ALSO_THIS_ONE UINT32_C(7)

#define SKIP_FALLBACK_A UINT32_C(5)
#define SKIP_FALLBACK_B UINT32_C(6 << 8)

#endif
