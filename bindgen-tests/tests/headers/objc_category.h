// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface Foo
-(void)method;
@end

@interface Foo (BarCategory)
-(void)categoryMethod;
@end
