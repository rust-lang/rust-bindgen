// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface Foo
@end

struct FooStruct {
  Foo *foo;
};

void fooFunc(Foo *foo);

static const Foo *kFoo;
