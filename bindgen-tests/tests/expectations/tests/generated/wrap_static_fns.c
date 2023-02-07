int foo__extern(void) asm("foo__extern");
int foo__extern() { return foo(); }
int bar__extern(void) asm("bar__extern");
int bar__extern() { return bar(); }
int takes_ptr__extern(int *arg) asm("takes_ptr__extern");
int takes_ptr__extern(int *arg) { return takes_ptr(arg); }
int takes_fn_ptr__extern(int (*f) (int)) asm("takes_fn_ptr__extern");
int takes_fn_ptr__extern(int (*f) (int)) { return takes_fn_ptr(f); }
int takes_fn__extern(int (f) (int)) asm("takes_fn__extern");
int takes_fn__extern(int (f) (int)) { return takes_fn(f); }
int takes_alias__extern(func f) asm("takes_alias__extern");
int takes_alias__extern(func f) { return takes_alias(f); }
int takes_qualified__extern(const int *const *arg) asm("takes_qualified__extern");
int takes_qualified__extern(const int *const *arg) { return takes_qualified(arg); }
