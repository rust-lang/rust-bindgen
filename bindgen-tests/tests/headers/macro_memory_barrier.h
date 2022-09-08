// bindgen-flags: --experimental --generate-fn-macros

#define MEMORY_BARRIER() __asm__ volatile ( "" ::: "memory" )
