//! Determining which types for which we can emit `#[derive(Debug)]`.

use super::{ConstrainResult, MonotoneFramework};
use std::collections::HashSet;
use std::collections::HashMap;
use ir::context::{BindgenContext, ItemId};
use ir::item::IsOpaque;
use ir::traversal::EdgeKind;
use ir::ty::RUST_DERIVE_IN_ARRAY_LIMIT;
use ir::ty::TypeKind;
use ir::comp::Field;
use ir::traversal::Trace;
use ir::comp::FieldMethods;
use ir::derive::CanTriviallyDeriveDebug;
use ir::comp::CompKind;

/// An analysis that finds for each IR item whether debug cannot be derived.
///
/// We use the monotone constraint function `cannot_derive_debug`, defined as
/// follows:
///
/// * If T is Opaque and layout of the type is known, get this layout as opaque
///   type and check whether it can be derived using trivial checks.
/// * If T is Array type, debug cannot be derived if the length of the array is
///   larger than the limit or the type of data the array contains cannot derive
///   debug.
/// * If T is a type alias, a templated alias or an indirection to another type,
///   debug cannot be derived if the type T refers to cannot be derived debug.
/// * If T is a compound type, debug cannot be derived if any of its base member
///   or field cannot be derived debug.
/// * If T is a pointer, T cannot be derived debug if T is a function pointer
///   and the function signature cannot be derived debug.
/// * If T is an instantiation of an abstract template definition, T cannot be
///   derived debug if any of the template arguments or template definition
///   cannot derive debug.
#[derive(Debug, Clone)]
pub struct CannotDeriveDebug<'ctx, 'gen>
    where 'gen: 'ctx
{
    ctx: &'ctx BindgenContext<'gen>,

    // The incremental result of this analysis's computation. Everything in this
    // set cannot derive debug.
    cannot_derive_debug: HashSet<ItemId>,

    // Dependencies saying that if a key ItemId has been inserted into the
    // `cannot_derive_debug` set, then each of the ids in Vec<ItemId> need to be
    // considered again.
    //
    // This is a subset of the natural IR graph with reversed edges, where we
    // only include the edges from the IR graph that can affect whether a type
    // can derive debug or not.
    dependencies: HashMap<ItemId, Vec<ItemId>>,
}

impl<'ctx, 'gen> CannotDeriveDebug<'ctx, 'gen> {
    fn consider_edge(kind: EdgeKind) -> bool {
        match kind {
            // These are the only edges that can affect whether a type can derive
            // debug or not.
            EdgeKind::BaseMember |
            EdgeKind::Field |
            EdgeKind::TypeReference |
            EdgeKind::VarType |
            EdgeKind::TemplateArgument |
            EdgeKind::TemplateDeclaration |
            EdgeKind::TemplateParameterDefinition => true,

            EdgeKind::Constructor |
            EdgeKind::Destructor |
            EdgeKind::FunctionReturn |
            EdgeKind::FunctionParameter |
            EdgeKind::InnerType |
            EdgeKind::InnerVar |
            EdgeKind::Method => false,
            EdgeKind::Generic => false,
        }
    }

    fn insert(&mut self, id: ItemId) -> ConstrainResult {
        let was_not_already_in_set = self.cannot_derive_debug.insert(id);
        assert!(
            was_not_already_in_set,
            "We shouldn't try and insert {:?} twice because if it was \
             already in the set, `constrain` should have exited early.",
            id
        );
        ConstrainResult::Changed
    }
}

impl<'ctx, 'gen> MonotoneFramework for CannotDeriveDebug<'ctx, 'gen> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext<'gen>;
    type Output = HashSet<ItemId>;

    fn new(ctx: &'ctx BindgenContext<'gen>) -> CannotDeriveDebug<'ctx, 'gen> {
        let cannot_derive_debug = HashSet::new();
        let mut dependencies = HashMap::new();

        for &item in ctx.whitelisted_items() {
            dependencies.entry(item).or_insert(vec![]);

            {
                // We reverse our natural IR graph edges to find dependencies
                // between nodes.
                item.trace(ctx, &mut |sub_item: ItemId, edge_kind| {
                    if ctx.whitelisted_items().contains(&sub_item) &&
                       Self::consider_edge(edge_kind) {
                           dependencies.entry(sub_item)
                               .or_insert(vec![])
                               .push(item);
                    }
                }, &());
            }
        }

        CannotDeriveDebug {
            ctx,
            cannot_derive_debug,
            dependencies,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        self.ctx.whitelisted_items().iter().cloned().collect()
    }

    fn constrain(&mut self, id: ItemId) -> ConstrainResult {
        if self.cannot_derive_debug.contains(&id) {
            return ConstrainResult::Same;
        }

        let item = self.ctx.resolve_item(id);
        let ty = match item.as_type() {
            None => return ConstrainResult::Same,
            Some(ty) => ty
        };

        if ty.is_opaque(self.ctx, item) {
            let layout_can_derive = ty.layout(self.ctx).map_or(true, |l| {
                l.opaque().can_trivially_derive_debug(self.ctx, ())
            });
            if layout_can_derive {
                return ConstrainResult::Same;
            } else {
                return self.insert(id);
            }
        }

        match *ty.kind() {
            // Handle the simple cases. These can derive debug without further
            // information.
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Int(..) |
            TypeKind::Float(..) |
            TypeKind::Complex(..) |
            TypeKind::Function(..) |
            TypeKind::Enum(..) |
            TypeKind::Reference(..) |
            TypeKind::BlockPointer |
            TypeKind::Named |
            TypeKind::UnresolvedTypeRef(..) |
            TypeKind::ObjCInterface(..) |
            TypeKind::ObjCId |
            TypeKind::ObjCSel => {
                ConstrainResult::Same
            }

            TypeKind::Array(t, len) => {
                if self.cannot_derive_debug.contains(&t) {
                    return self.insert(id);
                }

                if len <= RUST_DERIVE_IN_ARRAY_LIMIT {
                    ConstrainResult::Same
                } else {
                    self.insert(id)
                }
            }

            TypeKind::ResolvedTypeRef(t) |
            TypeKind::TemplateAlias(t, _) |
            TypeKind::Alias(t) => {
                if self.cannot_derive_debug.contains(&t) {
                    self.insert(id)
                } else {
                    ConstrainResult::Same
                }
            }

            TypeKind::Comp(ref info) => {
                assert!(
                    !info.has_non_type_template_params(),
                    "The early ty.is_opaque check should have handled this case"
                );

                if info.kind() == CompKind::Union {
                    if self.ctx.options().unstable_rust {
                        return self.insert(id);
                    }

                    if ty.layout(self.ctx)
                        .map_or(true,
                                |l| l.opaque().can_trivially_derive_debug(self.ctx, ())) {
                        return ConstrainResult::Same;
                    } else {
                        return self.insert(id);
                    }
                }

                let bases_cannot_derive = info.base_members()
                    .iter()
                    .any(|base| self.cannot_derive_debug.contains(&base.ty));
                if bases_cannot_derive {
                    return self.insert(id);
                }

                let fields_cannot_derive = info.fields()
                    .iter()
                    .any(|f| {
                        match *f {
                            Field::DataMember(ref data) => {
                                self.cannot_derive_debug.contains(&data.ty())
                            }
                            Field::Bitfields(ref bfu) => {
                                bfu.bitfields()
                                    .iter().any(|b| {
                                        self.cannot_derive_debug.contains(&b.ty())
                                    })
                            }
                        }
                    });
                if fields_cannot_derive {
                    return self.insert(id);
                }

                ConstrainResult::Same
            }

            TypeKind::Pointer(inner) => {
                let inner_type = self.ctx.resolve_type(inner).canonical_type(self.ctx);
                if let TypeKind::Function(ref sig) = *inner_type.kind() {
                    if !sig.can_trivially_derive_debug(&self.ctx, ()) {
                        return self.insert(id);
                    }
                }
                ConstrainResult::Same
            }

            TypeKind::TemplateInstantiation(ref template) => {
                let args_cannot_derive = template.template_arguments()
                    .iter()
                    .any(|arg| self.cannot_derive_debug.contains(&arg));
                if args_cannot_derive {
                    return self.insert(id);
                }

                assert!(
                    !template.template_definition().is_opaque(self.ctx, &()),
                    "The early ty.is_opaque check should have handled this case"
                );
                let def_cannot_derive = self.cannot_derive_debug
                    .contains(&template.template_definition());
                if def_cannot_derive {
                    return self.insert(id);
                }

                ConstrainResult::Same
            }

            TypeKind::Opaque => {
                unreachable!(
                    "The early ty.is_opaque check should have handled this case"
                )
            }
        }
    }

    fn each_depending_on<F>(&self, id: ItemId, mut f: F)
        where F: FnMut(ItemId),
    {
        if let Some(edges) = self.dependencies.get(&id) {
            for item in edges {
                trace!("enqueue {:?} into worklist", item);
                f(*item);
            }
        }
    }
}

impl<'ctx, 'gen> From<CannotDeriveDebug<'ctx, 'gen>> for HashSet<ItemId> {
    fn from(analysis: CannotDeriveDebug<'ctx, 'gen>) -> Self {
        analysis.cannot_derive_debug
    }
}
