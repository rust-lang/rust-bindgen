use support::assert_bind_eq;

#[test]
fn replace_type_alias_member() {
    assert_bind_eq(Default::default(), "headers/replace_type_alias_member.h", "");
}
