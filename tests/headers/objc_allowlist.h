// bindgen-flags: --objc-extern-crate --allowlist-type AllowlistMe --allowlist-type AllowlistMe_InterestingCategory -- -x objective-c
// bindgen-osx-only


// Protocol should be included, since it is used by the AllowlistMe
@protocol SomeProtocol
-(void)protocolMethod;
+(void)protocolClassMethod;
@end

// The allowlisted item
@interface AllowlistMe <SomeProtocol>
-(void)method;
+(void)classMethod;
@end

// This was also explicitly allowlisted
@interface AllowlistMe (InterestingCategory)
@end

// This was not automatically allowlisted
@interface AllowlistMe (IgnoredCategory)
@end

