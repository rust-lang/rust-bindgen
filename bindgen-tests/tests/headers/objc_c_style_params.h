// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface Foo
- (void)method;
- (void)multi:(int)a and:(int)b;
- (int)take:(int)t, int extra;
@end
