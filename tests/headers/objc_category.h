// bindgen-flags: --objc-extern-crate -- -x objective-c
// bindgen-osx-only

@interface Foo
-(void)method;
@end

@interface Foo (BarCategory)
-(void)categoryMethod;
@end
