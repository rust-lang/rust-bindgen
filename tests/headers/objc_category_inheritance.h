// bindgen-flags: --objc-extern-crate -- -x objective-c
// bindgen-osx-only

@interface Foo
@end

@interface Foo (BarCategory)
@end

@interface Bar: Foo
@end
