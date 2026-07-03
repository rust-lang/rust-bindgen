// bindgen-flags: -- -x objective-c -isystem tests/headers/support/objc-builtin-typedef-cycle

#include <system-objc.h>

@protocol Foo
- (id)foo;
@end
