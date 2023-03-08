//! Types and function used to emit pretty diagnostics for `bindgen`.
//!
//! The entry point of this module is the [`Diagnostic`] type.

use std::borrow::Cow;

use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, Slice as ExtSlice, Snippet},
};

pub(crate) use annotate_snippets::snippet::AnnotationType;

/// A `bindgen` diagnostic.
#[derive(Default)]
pub(crate) struct Diagnostic<'a> {
    title: Option<(Cow<'a, str>, AnnotationType)>,
    slices: Vec<Slice<'a>>,
    footer: Vec<(Cow<'a, str>, AnnotationType)>,
}

impl<'a> Diagnostic<'a> {
    /// Add a title to the diagnostic and set its type.
    pub(crate) fn with_title(
        &mut self,
        title: impl Into<Cow<'a, str>>,
        ty: AnnotationType,
    ) -> &mut Self {
        self.title = Some((title.into(), ty));
        self
    }

    /// Add a slice of source code to the diagnostic.
    pub(crate) fn add_slice(&mut self, slice: Slice<'a>) -> &mut Self {
        self.slices.push(slice);
        self
    }

    /// Add a footer annotation to the diagnostic. This annotation will have its own type.
    pub(crate) fn add_annotation(
        &mut self,
        msg: impl Into<Cow<'a, str>>,
        ty: AnnotationType,
    ) -> &mut Self {
        self.footer.push((msg.into(), ty));
        self
    }

    /// Print this diagnostic.
    ///
    /// The diagnostic is printed using `cargo:warning` if `bindgen` is being invoked by a build
    /// script or using `eprintln` otherwise.
    pub(crate) fn display(&self) {
        std::thread_local! {
            static INVOKED_BY_BUILD_SCRIPT: bool =  std::env::var_os("CARGO_CFG_TARGET_ARCH").is_some();
        }

        let mut title = None;
        let mut footer = vec![];
        let mut slices = vec![];
        if let Some((msg, ty)) = &self.title {
            title = Some(Annotation {
                id: None,
                label: Some(msg.as_ref()),
                annotation_type: *ty,
            })
        }

        for (msg, ty) in &self.footer {
            footer.push(Annotation {
                id: None,
                label: Some(msg.as_ref()),
                annotation_type: *ty,
            });
        }

        for slice in &self.slices {
            if let Some(source) = &slice.source {
                slices.push(ExtSlice {
                    source: source.as_ref(),
                    line_start: 0,
                    origin: None,
                    annotations: vec![],
                    fold: false,
                })
            }
        }

        let snippet = Snippet {
            title,
            footer,
            slices,
            opt: FormatOptions {
                color: true,
                ..Default::default()
            },
        };
        let dl = DisplayList::from(snippet);

        if INVOKED_BY_BUILD_SCRIPT.with(Clone::clone) {
            println!("cargo:warning={}", dl);
        } else {
            eprintln!("{}", dl);
        }
    }
}

/// A slice of source code.
#[derive(Default)]
pub(crate) struct Slice<'a> {
    source: Option<Cow<'a, str>>,
}

impl<'a> Slice<'a> {
    /// Set the source code.
    pub(crate) fn with_source(
        &mut self,
        source: impl Into<Cow<'a, str>>,
    ) -> &mut Self {
        self.source = Some(source.into());
        self
    }
}
