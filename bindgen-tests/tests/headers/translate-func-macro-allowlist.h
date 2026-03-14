// bindgen-flags: --translate-function-macros --allowlist-item "KEEP.*"

#define KEEP_ADD(x, y) ((x) + (y))
#define KEEP_FLAG(n) (1 << (n))
#define EXCLUDED(x) ((x) * 2)
