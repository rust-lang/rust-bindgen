// bindgen-flags: --allowlist-type="Struct" --allowlist-function="fun"

// no typedef should be emitted for `__uint64_t`
typedef unsigned long long __uint64_t;
typedef __uint64_t uint64_t;

uint64_t fun();
struct Struct {
	uint64_t field;
};
