// bindgen-flags: --clang-macro-fallback

#define BEFORE_DECL ((struct later *)3UL)
#define CONST_PTR ((const struct later *)4UL)

struct later;

typedef struct later *later_ptr;
#define TYPEDEF_PTR ((later_ptr)5UL)
#define MAP_FAILED ((void *)-1)
#define MAP_FAILED_ALIAS MAP_FAILED
#define MAP_FAILED_EQUALS_ITSELF (MAP_FAILED == MAP_FAILED)
#define REDEFINED_FROM_INT 1
#undef REDEFINED_FROM_INT
#define REDEFINED_FROM_INT ((void *)2)
#define REDEFINED_ALIAS REDEFINED_FROM_INT
