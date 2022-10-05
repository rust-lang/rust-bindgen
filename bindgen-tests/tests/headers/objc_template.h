// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface Foo<__covariant ObjectType>
- (ObjectType)get;
@end

@interface FooMultiGeneric<__covariant KeyType, __covariant ObjectType>
- (nullable ObjectType)objectForKey:(KeyType)key;
@end
