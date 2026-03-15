# Translating Function-Like Macros

By default, `bindgen` can only generate Rust constants for simple object-like C
macros (e.g., `#define FOO 42`). Function-like macros (macros with parameters)
are silently skipped.

The `--translate-function-macros` flag enables translating function-like C macros
into Rust `const fn` declarations.

## Basic Usage

**CLI:**

```console
bindgen input.h --translate-function-macros
```

**`build.rs`:**

```rust,ignore
let bindings = bindgen::Builder::default()
    .header("input.h")
    .translate_function_macros()
    .generate()
    .expect("Unable to generate bindings");
```

## What Gets Translated

Given:

```c
#define ADD(x, y)       ((x) + (y))
#define FLAG(n)         (1 << (n))
#define MAX(a, b)       ((a) > (b) ? (a) : (b))
```

`bindgen` generates:

```rust,ignore
pub const fn ADD(x: i64, y: i64) -> i64 { ((x) + (y)) }
pub const fn FLAG(n: i64) -> i64 { (1 << (n)) }
pub const fn MAX(a: i64, b: i64) -> i64 {
    (if (a) > (b) { (a) } else { (b) })
}
```

## Type Inference

When a macro body references object-like macro constants, `bindgen` infers the
value type from those constants. For example, if all referenced constants are
`u32`, the function uses `u32` for its parameters and return type.

Type inference runs after the entire header is parsed, so the order of `#define`s
does not matter. Forward references between function-like macros are also
supported — `bindgen` translates them in multiple passes until all dependencies
are resolved.

```c
#define _IOC_DIRSHIFT   30      // bindgen emits: pub const _IOC_DIRSHIFT: u32 = 30;
#define _IOC_TYPESHIFT  8
#define _IOC(dir, type, nr, size) \
    (((dir) << _IOC_DIRSHIFT) | ((type) << _IOC_TYPESHIFT) | (nr) | ((size) << 16))
```

Generates:

```rust,ignore
pub const fn _IOC(dir: u32, r#type: u32, nr: u32, size: u32) -> u32 {
    ((dir) << _IOC_DIRSHIFT) | ((r#type) << _IOC_TYPESHIFT) | (nr) | ((size) << 16)
}
```

When no constants are referenced, the type defaults to `i64`.

### Overriding the Type

Use the `func_macro_type` callback to override the inferred type:

```rust,ignore
impl bindgen::callbacks::ParseCallbacks for MyCallbacks {
    fn func_macro_type(&self, name: &str, _inferred: &str) -> Option<String> {
        if name == "MY_MACRO" {
            Some("u16".into())
        } else {
            None
        }
    }
}
```

## sizeof and Generic Type Parameters

When a macro parameter is used as a `sizeof` argument, it becomes a generic type
parameter:

```c
#define _IOR(type, nr, size)  _IOC(2, (type), (nr), sizeof(size))
```

Generates:

```rust,ignore
pub const fn _IOR<size>(r#type: u32, nr: u32) -> u32 {
    _IOC(2, (r#type), (nr), core::mem::size_of::<size>() as u32)
}
```

Called as `_IOR::<MyStruct>(b'M' as u32, 1)`.

Concrete types in `sizeof` are also supported:

```c
#define OFFSET(n)  ((n) + sizeof(int))
```

Generates `(n) + core::mem::size_of::<i32>() as i64`.

## C Casts

C-style casts like `(unsigned long)(x)` are translated to Rust `as` expressions:

```c
#define TO_U32(x)  ((unsigned int)(x))
```

Generates: `((x) as u32 as i64)`

The intermediate cast preserves truncation semantics, and the final `as` converts
back to the function's value type.

## Logical Operators

C's logical NOT `!`, AND `&&`, and OR `||` are translated with correct
integer truthiness semantics:

```c
#define NOT(x)    (!(x))
#define BOTH(x,y) ((x) && (y))
#define EITHER(x,y) ((x) || (y))
```

Generates:

```rust,ignore
pub const fn NOT(x: i64) -> i64 { (((x) as i64 == 0) as i64) }
pub const fn BOTH(x: i64, y: i64) -> i64 {
    (((((x) as i64) != 0) && (((y) as i64) != 0)) as i64)
}
```

C's `!` is logical NOT (0→1, nonzero→0), not Rust's bitwise NOT.
Each `&&`/`||` operand is wrapped with `!= 0` to convert integers to `bool`,
and the result is cast back to the value type (0 or 1, matching C semantics).

## Float Literals

When a macro body contains float literals (including exponent notation like
`1e3`), the value type is automatically inferred as `f64`:

```c
#define ADD_HALF(x) ((x) + 0.5f)
```

Generates: `pub const fn ADD_HALF(x: f64) -> f64 { ((x) + 0.5) }`

Macros that mix float literals with integer-only operators (`%`, `&`, `|`,
`^`, `<<`, `>>`) are skipped since those operators don't work on floats.

## Octal Literals

C octal literals (leading zero, e.g., `0644`) are converted to Rust octal
syntax (`0o644`). Without this conversion, `0644` would be interpreted as
decimal 644 in Rust instead of octal 644 (decimal 420).

## Composing with `--clang-macro-fallback`

Both flags can be used together:

- `--translate-function-macros` emits the function-like macro **definitions** as
  `const fn`
- `--clang-macro-fallback` evaluates object-like macros that **invoke**
  function-like macros to concrete values

```console
bindgen input.h --translate-function-macros --clang-macro-fallback
```

This gives you both the callable `const fn _IO(...)` and the evaluated
`pub const UI_DEV_CREATE: u32 = 21761`.

## What Gets Skipped

The following macros are silently skipped (a warning is logged at `RUST_LOG=warn`):

| Pattern | Reason |
|---------|--------|
| Variadic macros (`...`, `__VA_ARGS__`) | No `const fn` equivalent |
| `#` stringification / `##` token pasting | Preprocessor operations |
| Macros with `void*`, pointer types | Return type is integer-based |
| Statement macros (`do { } while(0)`) | Not expressions |
| `typeof` | No Rust equivalent |
| Typedef casts `(my_type)(x)` | Not expressible in `const fn` |
| Compiler builtins (`__asm__`, `__builtin_*`) | Not real functions |
| String literal bodies | Return type mismatch |
| Assignment operators (`+=`, `=`, etc.) | Produces `()`, not a value |
| C comma operator `(a, b)` | Becomes a Rust tuple |
| References to non-const globals | `static mut` requires `unsafe` |
