// bindgen-flags: --objc-extern-crate -- -x objective-c
// bindgen-osx-only

@interface Foo
// FIXME: We are not generating valid code for this
//        but at least we should not die
@property int (*func)(char, short, float);
@end
