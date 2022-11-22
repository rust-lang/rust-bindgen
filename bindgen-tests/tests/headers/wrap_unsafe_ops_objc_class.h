// bindgen-flags: --wrap-unsafe-ops -- -x objective-c
// bindgen-osx-only

@class Foo;

Foo* fooVar;

@interface Foo
-(void)method;
@end
