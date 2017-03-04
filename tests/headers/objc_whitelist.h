// bindgen-flags: --objc-extern-crate --whitelist-type WhitelistMe --whitelist-type WhitelistMe_InterestingCategory -- -x objective-c
// bindgen-osx-only


// Protocol should be included, since it is used by the WhitelistMe
@protocol SomeProtocol
-(void)protocolMethod;
@end

// The whitelisted item
@interface WhitelistMe <SomeProtocol>
-(void)method;
@end

// This was also explicitly whitelisted
@interface WhitelistMe (InterestingCategory)
@end

// This was not automatically whitelisted
@interface WhitelistMe (IgnoredCategory)
@end

