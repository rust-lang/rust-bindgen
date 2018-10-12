// bindgen-flags: --blacklist-identifier "someFunction" --blacklist-identifier "SOME_DEFUN" --blacklist-identifier "someVar"

void someFunction();
extern int someVar;
#define SOME_DEFUN 123
