// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface Foo
- (Class)take:(int)t;
- (Class)cls;
- (void)setCls:(Class)c;
@end
