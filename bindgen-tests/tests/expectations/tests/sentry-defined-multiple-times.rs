#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod whatever {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Wrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Wrapper_sentry {
            pub i_am_wrapper_sentry: ::std::os::raw::c_int,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct sentry {
            pub i_am_plain_sentry: bool,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            ["Size of sentry"][::std::mem::size_of::<sentry>() - 1usize];
            ["Alignment of sentry"][::std::mem::align_of::<sentry>() - 1usize];
            [
                "Offset of field: sentry::i_am_plain_sentry",
            ][::std::mem::offset_of!(sentry, i_am_plain_sentry) - 0usize];
        };
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct NotTemplateWrapper {
            pub _address: u8,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of NotTemplateWrapper",
            ][::std::mem::size_of::<NotTemplateWrapper>() - 1usize];
            [
                "Alignment of NotTemplateWrapper",
            ][::std::mem::align_of::<NotTemplateWrapper>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct NotTemplateWrapper_sentry {
            pub i_am_not_template_wrapper_sentry: ::std::os::raw::c_char,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of NotTemplateWrapper_sentry",
            ][::std::mem::size_of::<NotTemplateWrapper_sentry>() - 1usize];
            [
                "Alignment of NotTemplateWrapper_sentry",
            ][::std::mem::align_of::<NotTemplateWrapper_sentry>() - 1usize];
            [
                "Offset of field: NotTemplateWrapper_sentry::i_am_not_template_wrapper_sentry",
            ][::std::mem::offset_of!(
                NotTemplateWrapper_sentry, i_am_not_template_wrapper_sentry
            ) - 0usize];
        };
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InlineNotTemplateWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InlineNotTemplateWrapper_sentry {
            pub i_am_inline_not_template_wrapper_sentry: bool,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of InlineNotTemplateWrapper_sentry",
            ][::std::mem::size_of::<InlineNotTemplateWrapper_sentry>() - 1usize];
            [
                "Alignment of InlineNotTemplateWrapper_sentry",
            ][::std::mem::align_of::<InlineNotTemplateWrapper_sentry>() - 1usize];
            [
                "Offset of field: InlineNotTemplateWrapper_sentry::i_am_inline_not_template_wrapper_sentry",
            ][::std::mem::offset_of!(
                InlineNotTemplateWrapper_sentry, i_am_inline_not_template_wrapper_sentry
            ) - 0usize];
        };
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of InlineNotTemplateWrapper",
            ][::std::mem::size_of::<InlineNotTemplateWrapper>() - 1usize];
            [
                "Alignment of InlineNotTemplateWrapper",
            ][::std::mem::align_of::<InlineNotTemplateWrapper>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InlineTemplateWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct InlineTemplateWrapper_sentry {
            pub i_am_inline_template_wrapper_sentry: ::std::os::raw::c_int,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleWrapper_InnerDoubleWrapper {
            pub _address: u8,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of OuterDoubleWrapper_InnerDoubleWrapper",
            ][::std::mem::size_of::<OuterDoubleWrapper_InnerDoubleWrapper>() - 1usize];
            [
                "Alignment of OuterDoubleWrapper_InnerDoubleWrapper",
            ][::std::mem::align_of::<OuterDoubleWrapper_InnerDoubleWrapper>() - 1usize];
        };
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of OuterDoubleWrapper",
            ][::std::mem::size_of::<OuterDoubleWrapper>() - 1usize];
            [
                "Alignment of OuterDoubleWrapper",
            ][::std::mem::align_of::<OuterDoubleWrapper>() - 1usize];
        };
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleWrapper_InnerDoubleWrapper_sentry {
            pub i_am_double_wrapper_sentry: ::std::os::raw::c_int,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of OuterDoubleWrapper_InnerDoubleWrapper_sentry",
            ][::std::mem::size_of::<OuterDoubleWrapper_InnerDoubleWrapper_sentry>()
                - 4usize];
            [
                "Alignment of OuterDoubleWrapper_InnerDoubleWrapper_sentry",
            ][::std::mem::align_of::<OuterDoubleWrapper_InnerDoubleWrapper_sentry>()
                - 4usize];
            [
                "Offset of field: OuterDoubleWrapper_InnerDoubleWrapper_sentry::i_am_double_wrapper_sentry",
            ][::std::mem::offset_of!(
                OuterDoubleWrapper_InnerDoubleWrapper_sentry, i_am_double_wrapper_sentry
            ) - 0usize];
        };
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleInlineWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleInlineWrapper_InnerDoubleInlineWrapper {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry {
            pub i_am_double_wrapper_inline_sentry: ::std::os::raw::c_int,
        }
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry",
            ][::std::mem::size_of::<
                OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry,
            >() - 4usize];
            [
                "Alignment of OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry",
            ][::std::mem::align_of::<
                OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry,
            >() - 4usize];
            [
                "Offset of field: OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry::i_am_double_wrapper_inline_sentry",
            ][::std::mem::offset_of!(
                OuterDoubleInlineWrapper_InnerDoubleInlineWrapper_sentry,
                i_am_double_wrapper_inline_sentry
            ) - 0usize];
        };
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of OuterDoubleInlineWrapper_InnerDoubleInlineWrapper",
            ][::std::mem::size_of::<OuterDoubleInlineWrapper_InnerDoubleInlineWrapper>()
                - 1usize];
            [
                "Alignment of OuterDoubleInlineWrapper_InnerDoubleInlineWrapper",
            ][::std::mem::align_of::<OuterDoubleInlineWrapper_InnerDoubleInlineWrapper>()
                - 1usize];
        };
        #[allow(clippy::unnecessary_operation, clippy::identity_op)]
        const _: () = {
            [
                "Size of OuterDoubleInlineWrapper",
            ][::std::mem::size_of::<OuterDoubleInlineWrapper>() - 1usize];
            [
                "Alignment of OuterDoubleInlineWrapper",
            ][::std::mem::align_of::<OuterDoubleInlineWrapper>() - 1usize];
        };
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct OutsideNamespaceWrapper {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct OutsideNamespaceWrapper_sentry {
        pub i_am_outside_namespace_wrapper_sentry: ::std::os::raw::c_int,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct sentry {
        pub i_am_outside_namespace_sentry: ::std::os::raw::c_int,
    }
    #[allow(clippy::unnecessary_operation, clippy::identity_op)]
    const _: () = {
        ["Size of sentry"][::std::mem::size_of::<sentry>() - 4usize];
        ["Alignment of sentry"][::std::mem::align_of::<sentry>() - 4usize];
        [
            "Offset of field: sentry::i_am_outside_namespace_sentry",
        ][::std::mem::offset_of!(sentry, i_am_outside_namespace_sentry) - 0usize];
    };
}
