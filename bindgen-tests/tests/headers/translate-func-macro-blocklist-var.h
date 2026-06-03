// bindgen-flags: --translate-function-macros --blocklist-var BLOCK_ME

#define BLOCK_ME(x) ((x) + 1)
#define KEEP_ME(x) ((x) + 2)
