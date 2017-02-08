//! Helpers for code generation that need struct layout

use super::helpers::BlobTyBuilder;

use aster::struct_field::StructFieldBuilder;

use ir::comp::CompInfo;
use ir::context::BindgenContext;
use ir::layout::Layout;
use ir::ty::Type;
use std::cmp;
use std::mem;

use syntax::ast;

/// Trace the layout of struct.
pub struct StructLayoutTracker<'a, 'ctx: 'a> {
    ctx: &'a BindgenContext<'ctx>,
    comp: &'a CompInfo,
    latest_offset: usize,
    padding_count: usize,
    latest_field_layout: Option<Layout>,
    max_field_align: usize,
}

impl<'a, 'ctx> StructLayoutTracker<'a, 'ctx> {
    pub fn new(ctx: &'a BindgenContext<'ctx>, comp: &'a CompInfo) -> Self {
        StructLayoutTracker {
            ctx: ctx,
            comp: comp,
            latest_offset: 0,
            padding_count: 0,
            latest_field_layout: None,
            max_field_align: 0,
        }
    }

    pub fn saw_vtable(&mut self) {
        let ptr_size = mem::size_of::<*mut ()>();
        self.latest_offset += ptr_size;
        self.latest_field_layout = Some(Layout::new(ptr_size, ptr_size));
        self.max_field_align = ptr_size;
    }

    pub fn saw_base(&mut self, base_ty: &Type) {
        self.align_to_latest_field();

        if let Some(layout) = base_ty.layout(self.ctx) {
            self.latest_offset += self.padding_bytes(layout) + layout.size;
            self.latest_field_layout = Some(layout);
            self.max_field_align = cmp::max(self.max_field_align, layout.align);
        }
    }

    pub fn saw_bitfield(&mut self, layout: Layout) {
        self.align_to_latest_field();

        self.latest_offset += self.padding_bytes(layout) + layout.size;
        self.latest_field_layout = Some(layout);
        self.max_field_align = cmp::max(self.max_field_align, layout.align);
    }

    pub fn saw_union(&mut self, layout: Layout) {
        self.align_to_latest_field();

        self.latest_offset += self.padding_bytes(layout) + layout.size;
        self.latest_field_layout = Some(layout);
        self.max_field_align = cmp::max(self.max_field_align, layout.align);
    }

    pub fn pad_field(&mut self,
                     field_name: &str,
                     field_ty: &Type,
                     field_offset: Option<usize>)
                     -> Option<ast::StructField> {
        field_ty.layout(self.ctx).and_then(|field_layout| {
            self.align_to_latest_field();

            let padding_layout = if self.comp.packed() {
                None
            } else {
                let calculated_layout = field_ty.as_comp()
                    .and_then(|comp| comp.calc_layout(self.ctx))
                    .unwrap_or(field_layout);

                let align = cmp::min(calculated_layout.align, mem::size_of::<*mut ()>());

                let (padding_bytes, need_padding) = match field_offset {
                    Some(offset) if offset / 8  > self.latest_offset => {
                        (offset / 8  - self.latest_offset, true)
                    }
                    _ if field_layout.align != 0 => {
                        (self.padding_bytes(field_layout), (self.latest_offset % field_layout.align) != 0)
                    }
                    _ => {
                        (0, false)
                    }
                };

                self.latest_offset += padding_bytes;

                debug!("align field {} to {}/{} with {} padding bytes {:?}, calculated {:?}",
                    field_name,
                    self.latest_offset,
                    field_offset.unwrap_or(0) / 8,
                    padding_bytes,
                    field_layout,
                    calculated_layout);

                if need_padding &&
                   (padding_bytes > calculated_layout.align ||
                    field_layout.align > mem::size_of::<*mut ()>()) {
                    Some(Layout::new(padding_bytes, align))
                } else {
                    None
                }
            };

            self.latest_offset += field_ty.calc_size(self.ctx).unwrap_or(field_layout.size);

            self.latest_field_layout = Some(field_layout);
            self.max_field_align = cmp::max(self.max_field_align, field_layout.align);

            padding_layout.map(|layout| self.padding_field(layout))
        })
    }

    pub fn pad_struct(&mut self, layout: Layout) -> Option<ast::StructField> {
        if layout.size < self.latest_offset {
            warn!("calculate struct layout incorrect, too more {} bytes",
                  self.latest_offset - layout.size);

            None
        } else {
            let padding_bytes = layout.size - self.latest_offset;
            let struct_align = cmp::min(layout.align,
                                        mem::size_of::<*mut ()>());

            if padding_bytes > struct_align ||
               (layout.align > mem::size_of::<*mut ()>() && padding_bytes > 0) {
                let padding_align = if self.comp.packed() {
                    1
                } else {
                    cmp::min(1 << padding_bytes.trailing_zeros(),
                             mem::size_of::<*mut ()>())
                };

                Some(self.padding_field(Layout::new(padding_bytes, padding_align)))
            } else {
                None
            }
        }
    }

    pub fn align_struct(&self, layout: Layout) -> Option<ast::StructField> {
        if self.max_field_align < layout.align &&
           layout.align <= mem::size_of::<*mut ()>() {
            let ty = BlobTyBuilder::new(Layout::new(0, layout.align)).build();

            Some(StructFieldBuilder::named("__bindgen_align")
                .pub_()
                .build_ty(ty))
        } else {
            None
        }
    }

    fn padding_bytes(&self, layout: Layout) -> usize {
        if layout.align == 0 {
            warn!("try to padding bytes without layout");

            0
        } else if self.latest_offset % layout.align == 0 {
            0
        } else {
            layout.align - (self.latest_offset % layout.align)
        }
    }

    fn padding_field(&mut self, layout: Layout) -> ast::StructField {
        let ty = BlobTyBuilder::new(layout).build();
        let padding_count = self.padding_count;

        self.padding_count += 1;

        let padding_field_name = format!("__bindgen_padding_{}", padding_count);

        self.max_field_align = cmp::max(self.max_field_align, layout.align);

        StructFieldBuilder::named(padding_field_name).pub_().build_ty(ty)
    }

    fn align_to_latest_field(&mut self) {
        if self.comp.packed() {
            // skip to align field when packed
        } else if let Some(layout) = self.latest_field_layout {
            self.latest_offset += self.padding_bytes(layout);
        }
    }
}
