// bindgen-flags: --translate-function-macros --blocklist-item "BLOCKED.*"

#define ALLOWED(x) ((x) + 1)
#define BLOCKED_ADD(x, y) ((x) + (y))
#define BLOCKED_MUL(x, y) ((x) * (y))
