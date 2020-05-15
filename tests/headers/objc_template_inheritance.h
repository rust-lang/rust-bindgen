// bindgen-flags: --objc-extern-crate -- -x objective-c
// bindgen-osx-only

@interface Foo<__covariant ObjectType>
@end

@interface Bar<__covariant ObjectType>: Foo
@end
