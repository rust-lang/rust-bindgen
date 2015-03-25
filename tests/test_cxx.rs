use bindgen;
use bindgen::BindgenOptions;
use support::assert_bind_eq;

fn cxx_options() -> BindgenOptions {
    let mut options = BindgenOptions::default();
    options.rename_types = false;

    options
}

#[test]
fn test_cxx_class() {
    assert_bind_eq(cxx_options(), "headers/class.hpp", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct C {
            pub a: ::std::os::raw::c_int,
        }");
}

#[test]
fn test_cxx_template() {
    assert_bind_eq(cxx_options(), "headers/template.hpp", "
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct Foo<T> {
            pub m_member: T,
        }
        extern \"C\" {
            #[link_name = \"_Z3bar3FooIiE\"]
            pub fn bar(foo: Foo<::std::os::raw::c_int>);
        }");
}
