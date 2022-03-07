// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@protocol Foo
@end

@interface Foo <Foo>
@end

@interface Bar : Foo
@end
