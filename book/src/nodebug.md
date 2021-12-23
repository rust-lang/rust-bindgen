# Preventing the Derivation of `Debug`

`bindgen` will attempt to derive the `Debug` traits on a best-effort
basis. Sometimes, it might not understand that although adding `#[derive(Debug)]` to a translated type definition will compile, it still shouldn't do
that for reasons it can't know. In these cases, the `nodebug` annotation can be
used to prevent bindgen to autoderive the `Debug` traits for a type.

### Library

* [`bindgen::Builder::no_debug`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.no_debug)

### Command Line

* `--no-debug <regex>`

### Annotations

```c
/**
 * Although bindgen can't know, this enum is not safe to format the output.
 * the value may be combined with multiple bits in many C/C++ cases,
 * for example:
 *
 * <div rustbindgen nodebug></div>
 */
enum AVRounding {
    AV_ROUND_ZERO     = 0,
    AV_ROUND_INF      = 1,
    AV_ROUND_DOWN     = 2,
    AV_ROUND_UP       = 3,
    AV_ROUND_NEAR_INF = 5,
    AV_ROUND_PASS_MINMAX = 8192,
};

// Prototype
int64_t av_rescale_rnd(int64_t a, int64_t b, int64_t c, enum AVRounding) av_const;

// Call
int64_t pts = av_rescale_rnd(40000, 3600, 90000, AV_ROUND_NEAR_INF | AV_ROUND_PASS_MINMAX);
```
