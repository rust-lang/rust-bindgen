// bindgen-flags: -- -x objective-c
// bindgen-osx-only

@interface A
-(void)f:(int)arg1 as:(int)arg2;
-(void)crate:(int)self;
@end

@interface B

@property(nonatomic, retain) id type;

@end
