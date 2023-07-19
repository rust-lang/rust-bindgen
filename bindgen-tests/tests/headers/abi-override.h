// bindgen-flags: --override-abi=foo=fastcall --override-abi=bar=stdcall --override-abi=boo=efiapi --override-abi=foobar=efiapi --override-abi qux=system

void foo();
void bar();
void baz();
void qux();

typedef void (*boo)();
typedef void (*foobar)(boo boo);
