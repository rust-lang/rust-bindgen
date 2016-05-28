use support::assert_bind_eq;

#[test]
fn test_keywords() {
    assert_bind_eq(Default::default(), "headers/keywords.h", "
        extern \"C\" {
            #[link_name(name = \"u8\")]
            pub static mut u8_: ::std::os::raw::c_int;
            #[link_name(name = \"u16\")]
            pub static mut u16_: ::std::os::raw::c_int;
            #[link_name(name = \"u32\")]
            pub static mut u32_: ::std::os::raw::c_int;
            #[link_name(name = \"u64\")]
            pub static mut u64_: ::std::os::raw::c_int;
            #[link_name(name = \"i8\")]
            pub static mut i8_: ::std::os::raw::c_int;
            #[link_name(name = \"i16\")]
            pub static mut i16_: ::std::os::raw::c_int;
            #[link_name(name = \"i32\")]
            pub static mut i32_: ::std::os::raw::c_int;
            #[link_name(name = \"i64\")]
            pub static mut i64_: ::std::os::raw::c_int;
            #[link_name(name = \"f32\")]
            pub static mut f32_: ::std::os::raw::c_int;
            #[link_name(name = \"f64\")]
            pub static mut f64_: ::std::os::raw::c_int;
            #[link_name(name = \"usize\")]
            pub static mut usize_: ::std::os::raw::c_int;
            #[link_name(name = \"isize\")]
            pub static mut isize_: ::std::os::raw::c_int;
            #[link_name(name = \"Self\")]
            pub static mut Self_: ::std::os::raw::c_int;
            #[link_name(name = \"self\")]
            pub static mut self_: ::std::os::raw::c_int;
            #[link_name(name = \"as\")]
            pub static mut as_: ::std::os::raw::c_int;
            #[link_name(name = \"box\")]
            pub static mut box_: ::std::os::raw::c_int;
            #[link_name(name = \"crate\")]
            pub static mut crate_: ::std::os::raw::c_int;
            #[link_name(name = \"false\")]
            pub static mut false_: ::std::os::raw::c_int;
            #[link_name(name = \"fn\")]
            pub static mut fn_: ::std::os::raw::c_int;
            #[link_name(name = \"impl\")]
            pub static mut impl_: ::std::os::raw::c_int;
            #[link_name(name = \"in\")]
            pub static mut in_: ::std::os::raw::c_int;
            #[link_name(name = \"let\")]
            pub static mut let_: ::std::os::raw::c_int;
            #[link_name(name = \"loop\")]
            pub static mut loop_: ::std::os::raw::c_int;
            #[link_name(name = \"match\")]
            pub static mut match_: ::std::os::raw::c_int;
            #[link_name(name = \"mod\")]
            pub static mut mod_: ::std::os::raw::c_int;
            #[link_name(name = \"move\")]
            pub static mut move_: ::std::os::raw::c_int;
            #[link_name(name = \"mut\")]
            pub static mut mut_: ::std::os::raw::c_int;
            #[link_name(name = \"pub\")]
            pub static mut pub_: ::std::os::raw::c_int;
            #[link_name(name = \"ref\")]
            pub static mut ref_: ::std::os::raw::c_int;
            #[link_name(name = \"super\")]
            pub static mut super_: ::std::os::raw::c_int;
            #[link_name(name = \"trait\")]
            pub static mut trait_: ::std::os::raw::c_int;
            #[link_name(name = \"true\")]
            pub static mut true_: ::std::os::raw::c_int;
            #[link_name(name = \"type\")]
            pub static mut type_: ::std::os::raw::c_int;
            #[link_name(name = \"unsafe\")]
            pub static mut unsafe_: ::std::os::raw::c_int;
            #[link_name(name = \"use\")]
            pub static mut use_: ::std::os::raw::c_int;
            #[link_name(name = \"where\")]
            pub static mut where_: ::std::os::raw::c_int;
            #[link_name(name = \"abstract\")]
            pub static mut abstract_: ::std::os::raw::c_int;
            #[link_name(name = \"alignof\")]
            pub static mut alignof_: ::std::os::raw::c_int;
            #[link_name(name = \"become\")]
            pub static mut become_: ::std::os::raw::c_int;
            #[link_name(name = \"final\")]
            pub static mut final_: ::std::os::raw::c_int;
            #[link_name(name = \"macro\")]
            pub static mut macro_: ::std::os::raw::c_int;
            #[link_name(name = \"offsetof\")]
            pub static mut offsetof_: ::std::os::raw::c_int;
            #[link_name(name = \"override\")]
            pub static mut override_: ::std::os::raw::c_int;
            #[link_name(name = \"priv\")]
            pub static mut priv_: ::std::os::raw::c_int;
            #[link_name(name = \"proc\")]
            pub static mut proc_: ::std::os::raw::c_int;
            #[link_name(name = \"pure\")]
            pub static mut pure_: ::std::os::raw::c_int;
            #[link_name(name = \"unsized\")]
            pub static mut unsized_: ::std::os::raw::c_int;
            #[link_name(name = \"virtual\")]
            pub static mut virtual_: ::std::os::raw::c_int;
            #[link_name(name = \"yield\")]
            pub static mut yield_: ::std::os::raw::c_int;
        }
    ");
}
