// bindgen-flags: --enable-function-attribute-detection
#include "issue-3496/noreturn.h"

#define DEPRECATED __attribute__((deprecated("foo")))
#define NORETURN _Noreturn

DEPRECATED int foo(void);
NORETURN void bar(void);
OTHER_FILE_NORETURN void baz(void);
