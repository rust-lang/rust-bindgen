// bindgen-flags: -- -x objective-c
// bindgen-osx-only

__attribute__((objc_root_class)) @interface Foo
- (int)take$foo:(int)t;
- (int)ok:(int)a and$more:(int)b;
- (int)fine:(int)t;
@end
