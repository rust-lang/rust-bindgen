// bindgen-flags: -- -x objective-c
// bindgen-osx-only

__attribute__((objc_root_class))
@interface Base
@end

__attribute__((objc_runtime_name("RuntimeFoo")))
@interface Foo : Base
+ (int)make;
- (int)value;
@end

@interface Plain : Base
+ (int)make;
@end
