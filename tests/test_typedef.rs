use support::assert_bind_eq;

#[test]
fn typedef_same_name() {
    assert_bind_eq(Default::default(), "headers/typedef_same_name.h", "
        pub enum SameS {  }
        pub enum SameE {  }
        pub enum SameU {  }
    ");
}
