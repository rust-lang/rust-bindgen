// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface Bar
@end

@interface Foo
+ (Bar*)methodReturningBar;
- (void)methodUsingBar:(Bar *)my_bar;
@end
