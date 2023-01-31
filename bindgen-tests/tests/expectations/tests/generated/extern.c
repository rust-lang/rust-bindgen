int foo__extern() asm("foo__extern");
foo__extern() { return foo(); }
int bar__extern() asm("bar__extern");
bar__extern() { return bar(); }
