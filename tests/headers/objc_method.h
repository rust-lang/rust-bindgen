// bindgen-flags: --objc-extern-crate -- -x objective-c
// bindgen-osx-only

@interface Foo
- (void)method;
- (void)methodWithInt:(int)foo;
- (void)methodWithFoo:(Foo*)foo;
- (int)methodReturningInt;
- (Foo*)methodReturningFoo;
- (void)methodWithArg1:(int)intvalue andArg2:(char*)ptr andArg3:(float)floatvalue;
- (instancetype)methodWithAndWithoutKeywords:(int)arg1
                                    arg2Name:(float)arg2
                                            :(float)arg3
                                    arg4Name:(int)arg4;
@end
