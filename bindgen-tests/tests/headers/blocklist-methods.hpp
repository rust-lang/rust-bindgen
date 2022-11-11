// bindgen-flags: --blocklist-function="Foo_ba.*"

class Foo {
    public:
        int foo();
        int bar();
        int baz();
};
