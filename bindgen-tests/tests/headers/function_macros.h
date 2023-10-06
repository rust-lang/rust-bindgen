// bindgen-flags: --experimental --macro-function "SIMPLE" --macro-function "INDIRECT_SIMPLE" --macro-function "f32 COMPLEX(u32)" --macro-function "f32 INDIRECT_COMPLEX(u32)" --macro-function "f32 CONDITIONAL_COMPLEX(bool, u32)"

void simple(void);

#define SIMPLE simple()
#define INDIRECT_SIMPLE SIMPLE

float complex(int);

#define COMPLEX(x) complex(x)
#define INDIRECT_COMPLEX(x) COMPLEX(x)

#define CONDITIONAL_COMPLEX(condition, x) ((condition) ? COMPLEX(x) : 0)

#define SIMPLE_NOT_CONFIGURED simple()
