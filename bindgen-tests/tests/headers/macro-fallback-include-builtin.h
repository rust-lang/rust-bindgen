// bindgen-flags: --clang-macro-fallback --allowlist-item TEST
// Ensures that the fallback path for macros doesn't silently
// fail because of builtin headers not being found.

#include <stddef.h>

#define TEST ((size_t)42)
