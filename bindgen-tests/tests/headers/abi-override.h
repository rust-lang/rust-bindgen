// bindgen-flags: --override-abi=foo=fastcall --override-abi=bar=stdcall --override-abi=boo=efiapi --override-abi=foobar=efiapi

void foo();
void bar();
void baz();

typedef void (*boo)();
typedef void (*foobar)(boo boo);
