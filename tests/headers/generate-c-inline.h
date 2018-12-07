// bindgen-flags: --generate-inline-functions

typedef struct {
    int foo;
} state_t;

static inline state_t state(state_t *s) { return *s; }

inline static int foo(const int x, const int y) { return x + y; }
static int bar(const int x, const int y) { return x - y; }
inline static void nop() { return; }
