// bindgen-flags: --objc-extern-crate -- -x objective-c
// bindgen-osx-only

@interface Foo<__covariant ObjectType>
- (ObjectType)get;
@end
