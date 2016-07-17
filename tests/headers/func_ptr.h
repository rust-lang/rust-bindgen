int (*foo) (int x, int y);

void takes_argdef(void proc());
void takes_argdef_ptr(void (*proc)());
void takes_argdef_ptr_ptr(void (**proc)());

// type with no indirection
typedef void fn_t();
void takes_tdef(fn_t proc);
void takes_tdef_ptr(fn_t* proc);
void takes_tdef_ptr_ptr(fn_t** proc);

// types with one indirection, should be the same as no indirection
typedef void (*fnptr_t)();
void takes_tdefptr(fnptr_t proc);
void takes_tdefptr_ptr(fnptr_t* proc);
void takes_tdefptr_ptr_ptr(fnptr_t** proc);

typedef fn_t *fntptr_t;
void takes_tdeftptr(fntptr_t proc);
void takes_tdeftptr_ptr(fntptr_t* proc);
void takes_tdeftptr_ptr_ptr(fntptr_t** proc);

// types with two indirection, effectively being one indirection
typedef void (**fnptrptr_t)();
void takes_tdefptrptr(fnptrptr_t proc);
void takes_tdefptrptr_ptr(fnptrptr_t* proc);
void takes_tdefptrptr_ptr_ptr(fnptrptr_t** proc);

typedef fntptr_t *fntptrptr_t;
void takes_tdeftptrtptr(fntptrptr_t proc);
void takes_tdeftptrtptr_ptr(fntptrptr_t* proc);
void takes_tdeftptrtptr_ptr_ptr(fntptrptr_t** proc);

typedef fn_t **fntptrtptr_t;
void takes_tdeftptrtptr(fntptrtptr_t proc);
void takes_tdeftptrtptr_ptr(fntptrtptr_t* proc);
void takes_tdeftptrtptr_ptr_ptr(fntptrtptr_t** proc);
