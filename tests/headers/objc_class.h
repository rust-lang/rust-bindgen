// bindgen-flags: --objc-extern-crate -- -x objective-c
// bindgen-osx-only

@class Foo;

Foo* fooVar;

@interface Foo
-(void)method;
@end
