// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface Foo
- (nullable int*)nullableReturnType;
- (nonnull int*)nonnullableReturnType;
- (int *)unspecifiedNullabilityReturnType;
//- (int *)unspecifiedNullabilityReturnType:(Foo*)foo;
@end

