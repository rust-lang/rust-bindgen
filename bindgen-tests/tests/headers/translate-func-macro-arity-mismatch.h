// bindgen-flags: --translate-function-macros

// Wrong-arity calls should cause the caller macro to be skipped,
// not emitted as uncompilable Rust (E0061).

#define B(x) ((x) + 1)

// A() calls B() with 0 args, but B expects 1.
#define A() B()

// C(x) calls B(x, x) with 2 args, but B expects 1.
#define C(x) B(x, x)
