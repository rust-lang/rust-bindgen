// bindgen-flags: --objc-extern-crate -- -x objective-c
// bindgen-osx-only

@interface Foo<__covariant ObjectType>
- (ObjectType)get;
@end

@interface Foo<__covariant ObjectType> (Baz)
- (ObjectType)get;
@end

@interface Bar<__covariant ObjectType>: Foo
- (ObjectType)get;
@end
