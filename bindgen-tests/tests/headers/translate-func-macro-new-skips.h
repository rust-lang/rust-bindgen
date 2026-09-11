// bindgen-flags: --translate-function-macros

// These should all be skipped (not appear in output).

// String literal body (Finding 5).
#define STRING_MACRO(x) "hello"

// Assignment operators (Finding 9).
#define ASSIGN_LIKE(x) ((x) += 1)

// Comma operator (Finding 4).
#define COMMA(a, b) ((a), (b))

// Compiler builtins (Finding 1).
#define ASMNAME(cname) __asm__(cname)

// This should still appear.
#define GOOD(x) ((x) + 1)
