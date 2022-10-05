// bindgen-flags: --enable-cxx-namespaces -- -std=c++11

// `Wrapper::sentry` and `sentry` should be emitted as `Wrapper_sentry` and
// `sentry` respectively, but instead `Wrapper::sentry` is named just `sentry`
// which leads to compilation errors.
//
// Note: if there is no namespace, then we don't run into problems. Similarly,
// making the `Wrapper::sentry` definition inline in `Wrapper`, rather than
// declared inline with an out of line definition, makes the problem go away as
// well.

namespace whatever {
    template <typename, typename>
    class Wrapper {
        // Declaration of Wrapper::sentry
        class sentry;
    };

    // Definition of Wrapper::sentry
    template <typename f, typename h>
    class Wrapper<f, h>::sentry {
        int i_am_wrapper_sentry;
    };

    class sentry {
        bool i_am_plain_sentry;
    };

    // Ok, that was the original bug report. While we're here, let's just try
    // lots of different things that could go wrong and make sure we handle them
    // right.

    class NotTemplateWrapper {
        class sentry;
    };

    class NotTemplateWrapper::sentry {
        char i_am_not_template_wrapper_sentry;
    };

    class InlineNotTemplateWrapper {
        class sentry {
            bool i_am_inline_not_template_wrapper_sentry;
        };
    };

    template <typename, typename>
    class InlineTemplateWrapper {
        class sentry {
            int i_am_inline_template_wrapper_sentry;
        };
    };

    class OuterDoubleWrapper {
        class InnerDoubleWrapper {
            class sentry;
        };
    };

    class OuterDoubleWrapper::InnerDoubleWrapper::sentry {
        int i_am_double_wrapper_sentry;
    };

    class OuterDoubleInlineWrapper {
        class InnerDoubleInlineWrapper {
            class sentry {
                int i_am_double_wrapper_inline_sentry;
            };
        };
    };
}

template <typename, typename>
class OutsideNamespaceWrapper {
    class sentry;
};

template <typename f, typename h>
class OutsideNamespaceWrapper<f, h>::sentry {
    int i_am_outside_namespace_wrapper_sentry;
};

class sentry {
    int i_am_outside_namespace_sentry;
};
