// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface Foo
@property int (*func)(char, short, float);
@end
