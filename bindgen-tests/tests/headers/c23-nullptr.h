// bindgen-flags: --allowlist-type "nullptr_t|NullAlias|NullHolder" --allowlist-function "ident|is_null" -- -std=c2x
// bindgen-min-clang-version: 17

#include <stddef.h>

typedef nullptr_t NullAlias;
struct NullHolder { nullptr_t value; };
nullptr_t ident(nullptr_t value);
int is_null(NullAlias value);
