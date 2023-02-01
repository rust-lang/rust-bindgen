int foo__extern() asm("foo__extern");
int foo__extern() { return foo(); }
int bar__extern() asm("bar__extern");
int bar__extern() { return bar(); }
int takes_ptr__extern(int* arg) asm("takes_ptr__extern");
int takes_ptr__extern(int*arg) { return takes_ptr(arg); }
