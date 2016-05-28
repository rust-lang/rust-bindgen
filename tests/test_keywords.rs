use support::assert_bind_eq;

#[test]
fn test_keywords() {
    assert_bind_eq(Default::default(), "headers/keywords.h", "
        extern \"C\" {
            #[link_name(name = \"u8\")]
            pub static mut _u8: ::std::os::raw::c_int;
            #[link_name(name = \"u16\")]
            pub static mut _u16: ::std::os::raw::c_int;
            #[link_name(name = \"u32\")]
            pub static mut _u32: ::std::os::raw::c_int;
            #[link_name(name = \"u64\")]
            pub static mut _u64: ::std::os::raw::c_int;
            #[link_name(name = \"i8\")]
            pub static mut _i8: ::std::os::raw::c_int;
            #[link_name(name = \"i16\")]
            pub static mut _i16: ::std::os::raw::c_int;
            #[link_name(name = \"i32\")]
            pub static mut _i32: ::std::os::raw::c_int;
            #[link_name(name = \"i64\")]
            pub static mut _i64: ::std::os::raw::c_int;
            #[link_name(name = \"f32\")]
            pub static mut _f32: ::std::os::raw::c_int;
            #[link_name(name = \"f64\")]
            pub static mut _f64: ::std::os::raw::c_int;
            #[link_name(name = \"usize\")]
            pub static mut _usize: ::std::os::raw::c_int;
            #[link_name(name = \"isize\")]
            pub static mut _isize: ::std::os::raw::c_int;
            #[link_name(name = \"Self\")]
            pub static mut _Self: ::std::os::raw::c_int;
            #[link_name(name = \"self\")]
            pub static mut _self: ::std::os::raw::c_int;
            #[link_name(name = \"as\")]
            pub static mut _as: ::std::os::raw::c_int;
            #[link_name(name = \"box\")]
            pub static mut _box: ::std::os::raw::c_int;
            #[link_name(name = \"crate\")]
            pub static mut _crate: ::std::os::raw::c_int;
            #[link_name(name = \"false\")]
            pub static mut _false: ::std::os::raw::c_int;
            #[link_name(name = \"fn\")]
            pub static mut _fn: ::std::os::raw::c_int;
            #[link_name(name = \"impl\")]
            pub static mut _impl: ::std::os::raw::c_int;
            #[link_name(name = \"in\")]
            pub static mut _in: ::std::os::raw::c_int;
            #[link_name(name = \"let\")]
            pub static mut _let: ::std::os::raw::c_int;
            #[link_name(name = \"loop\")]
            pub static mut _loop: ::std::os::raw::c_int;
            #[link_name(name = \"match\")]
            pub static mut _match: ::std::os::raw::c_int;
            #[link_name(name = \"mod\")]
            pub static mut _mod: ::std::os::raw::c_int;
            #[link_name(name = \"move\")]
            pub static mut _move: ::std::os::raw::c_int;
            #[link_name(name = \"mut\")]
            pub static mut _mut: ::std::os::raw::c_int;
            #[link_name(name = \"pub\")]
            pub static mut _pub: ::std::os::raw::c_int;
            #[link_name(name = \"ref\")]
            pub static mut _ref: ::std::os::raw::c_int;
            #[link_name(name = \"super\")]
            pub static mut _super: ::std::os::raw::c_int;
            #[link_name(name = \"trait\")]
            pub static mut _trait: ::std::os::raw::c_int;
            #[link_name(name = \"true\")]
            pub static mut _true: ::std::os::raw::c_int;
            #[link_name(name = \"type\")]
            pub static mut _type: ::std::os::raw::c_int;
            #[link_name(name = \"unsafe\")]
            pub static mut _unsafe: ::std::os::raw::c_int;
            #[link_name(name = \"use\")]
            pub static mut _use: ::std::os::raw::c_int;
            #[link_name(name = \"where\")]
            pub static mut _where: ::std::os::raw::c_int;
            #[link_name(name = \"abstract\")]
            pub static mut _abstract: ::std::os::raw::c_int;
            #[link_name(name = \"alignof\")]
            pub static mut _alignof: ::std::os::raw::c_int;
            #[link_name(name = \"become\")]
            pub static mut _become: ::std::os::raw::c_int;
            #[link_name(name = \"final\")]
            pub static mut _final: ::std::os::raw::c_int;
            #[link_name(name = \"macro\")]
            pub static mut _macro: ::std::os::raw::c_int;
            #[link_name(name = \"offsetof\")]
            pub static mut _offsetof: ::std::os::raw::c_int;
            #[link_name(name = \"override\")]
            pub static mut _override: ::std::os::raw::c_int;
            #[link_name(name = \"priv\")]
            pub static mut _priv: ::std::os::raw::c_int;
            #[link_name(name = \"proc\")]
            pub static mut _proc: ::std::os::raw::c_int;
            #[link_name(name = \"pure\")]
            pub static mut _pure: ::std::os::raw::c_int;
            #[link_name(name = \"unsized\")]
            pub static mut _unsized: ::std::os::raw::c_int;
            #[link_name(name = \"virtual\")]
            pub static mut _virtual: ::std::os::raw::c_int;
            #[link_name(name = \"yield\")]
            pub static mut _yield: ::std::os::raw::c_int;
        }
    ");
}
