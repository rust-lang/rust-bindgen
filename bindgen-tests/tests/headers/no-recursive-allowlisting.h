// bindgen-flags: --no-recursive-allowlist --allowlist-type "Foo" --raw-line "pub enum Bar {}" --rustified-enum ".*"

struct Bar;

struct Foo {
  struct Bar* baz;
};
