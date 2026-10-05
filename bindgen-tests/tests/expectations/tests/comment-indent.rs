#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    /// This is a multi-line doc comment.
    ///
    /// This class is really really interesting, look!
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Foo {
        pub _address: u8,
    }
    /// This nested class is also a multi-line doc comment.
    ///
    /// This class is not so interesting, but worth a bit of docs too!
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Foo_Bar {
        pub _address: u8,
    }
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of Foo_Bar"][::std::mem::size_of::<Foo_Bar>() - 1usize];
        ["Alignment of Foo_Bar"][::std::mem::align_of::<Foo_Bar>() - 1usize];
    };
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of Foo"][::std::mem::size_of::<Foo>() - 1usize];
        ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 1usize];
    };
    pub mod test {
        #[allow(unused_imports)]
        use self::super::super::root;
        /// I'm in a namespace, and thus I may be on a rust module, most of the time.
        /// My documentation is pretty extensive, I guess.
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Baz {
            /// This member is plain awesome, just amazing.
            ///
            /// It also has super-extensive docs, with even a nice ascii-art diagram.
            ///
            /// +------+          +-------+
            /// | foo  |   ---->  | bar   |
            /// +------+          +-------+
            pub member: ::std::os::raw::c_int,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of Baz"][::std::mem::size_of::<Baz>() - 4usize];
            ["Alignment of Baz"][::std::mem::align_of::<Baz>() - 4usize];
            [
                "Offset of field: Baz::member",
            ][::std::mem::offset_of!(Baz, member) - 0usize];
        };
        /// I'm in an inline namespace, and as such I shouldn't get generated inside
        /// a rust module, except when the relevant option is specified. Also, this
        /// comment shouldn't be misaligned.
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InInlineNS {
            pub _address: u8,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of InInlineNS"][::std::mem::size_of::<InInlineNS>() - 1usize];
            ["Alignment of InInlineNS"][::std::mem::align_of::<InInlineNS>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Bazz {
            pub _address: u8,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of Bazz"][::std::mem::size_of::<Bazz>() - 1usize];
            ["Alignment of Bazz"][::std::mem::align_of::<Bazz>() - 1usize];
        };
    }
}
