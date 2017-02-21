// bindgen-flags: --objc-extern-crate -- -x objective-c
// bindgen-osx-only

@protocol Foo
@end

@interface Foo <Foo>
@end
