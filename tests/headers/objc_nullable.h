// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface Foo
- (nullable int*)nullableReturnType;
- (nonnull int*)nonnullableReturnType;
- (int *)unspecifiedNullabilityReturnType;
- (void)nullableParameter:(nullable Foo*)foo;
- (void)nonnullableParameter:(nonnull Foo*)foo;
@property (assign, nonnull) Foo *nonnullFoo;
@property (assign, nullable) Foo *nullableFoo;
@end

