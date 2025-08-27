//! Intermediate representation for the physical layout of some type.

use crate::ir::context::BindgenContext;

/// A type that represents the struct layout of a type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Layout {
    /// The size (in bytes) of this layout.
    pub(crate) size: usize,
    /// The alignment (in bytes) of this layout.
    pub(crate) align: usize,
    /// Whether this layout's members are packed or not.
    pub(crate) packed: bool,
}

#[test]
fn test_layout_for_size() {
    use std::mem::size_of;
    let ptr_size = size_of::<*mut ()>();
    assert_eq!(
        Layout::for_size_internal(ptr_size, ptr_size),
        Layout::new(ptr_size, ptr_size)
    );
    assert_eq!(
        Layout::for_size_internal(ptr_size, 3 * ptr_size),
        Layout::new(3 * ptr_size, ptr_size)
    );
}

impl Layout {
    /// Gets the integer type name for a given known size.
    pub(crate) fn known_type_for_size(size: usize) -> Option<syn::Type> {
        Some(match size {
            16 => syn::parse_quote! { u128 },
            8 => syn::parse_quote! { u64 },
            4 => syn::parse_quote! { u32 },
            2 => syn::parse_quote! { u16 },
            1 => syn::parse_quote! { u8 },
            _ => return None,
        })
    }

    /// Construct a new `Layout` with the given `size` and `align`. It is not
    /// packed.
    pub(crate) fn new(size: usize, align: usize) -> Self {
        Layout {
            size,
            align,
            packed: false,
        }
    }

    fn for_size_internal(ptr_size: usize, size: usize) -> Self {
        let mut next_align = 2;
        while size % next_align == 0 && next_align <= ptr_size {
            next_align *= 2;
        }
        Layout {
            size,
            align: next_align / 2,
            packed: false,
        }
    }

    /// Creates a non-packed layout for a given size, trying to use the maximum
    /// alignment possible.
    pub(crate) fn for_size(ctx: &BindgenContext, size: usize) -> Self {
        Self::for_size_internal(ctx.target_pointer_size(), size)
    }
}
