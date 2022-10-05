// bindgen-flags: --objc-extern-crate --blocklist-item ISomeClass::class_ambiguouslyBlockedMethod --blocklist-item ISomeClass::blockedInstanceMethod -- -x objective-c
// bindgen-osx-only

@interface SomeClass
+ (void)ambiguouslyBlockedMethod;
- (void)ambiguouslyBlockedMethod;
- (void)instanceMethod;
- (void)blockedInstanceMethod;
@end
