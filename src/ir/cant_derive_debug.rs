//! Determining which types for which we can emit `#[derive(Debug)]`.
use super::analysis::MonotoneFramework;
use ir::context::{BindgenContext, ItemId};
use ir::item::ItemSet;
use std::collections::HashSet;
use std::collections::HashMap;
use ir::traversal::EdgeKind;
use ir::ty::RUST_DERIVE_IN_ARRAY_LIMIT;
use ir::ty::TypeKind;
use ir::comp::Field;
use ir::traversal::Trace;
use ir::comp::FieldMethods;
use ir::layout::Layout;
use ir::derive::CanTriviallyDeriveDebug;
use ir::comp::CompKind;

/// An analysis that finds for each IR item whether debug cannot be derived.
///
/// We use the monotone constraint function `cant_derive_debug`, defined as
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
pub struct CantDeriveDebugAnalysis<'ctx, 'gen>
    where 'gen: 'ctx
{
    ctx: &'ctx BindgenContext<'gen>,

    // The incremental result of this analysis's computation. Everything in this
    // set cannot derive debug.
    cant_derive_debug: HashSet<ItemId>,

    // Dependencies saying that if a key ItemId has been inserted into the
    // `cant_derive_debug` set, then each of the ids in Vec<ItemId> need to be
    // considered again.
    //
    // This is a subset of the natural IR graph with reversed edges, where we
    // only include the edges from the IR graph that can affect whether a type
    // can derive debug or not.
    dependencies: HashMap<ItemId, Vec<ItemId>>,
}

impl<'ctx, 'gen> CantDeriveDebugAnalysis<'ctx, 'gen> {
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

    fn insert(&mut self, id: ItemId) -> bool {
        let was_already_in = self.cant_derive_debug.insert(id);
        assert!(
            was_already_in,
            format!("We shouldn't try and insert twice because if it was already in the set, \
             `constrain` would have exited early.: {:?}", id)
        );
        true
    }
}

impl<'ctx, 'gen> MonotoneFramework for CantDeriveDebugAnalysis<'ctx, 'gen> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext<'gen>;
    type Output = HashSet<ItemId>;

    fn new(ctx: &'ctx BindgenContext<'gen>) -> CantDeriveDebugAnalysis<'ctx, 'gen> {
        let cant_derive_debug = HashSet::new();
        let mut dependencies = HashMap::new();
        let whitelisted_items: HashSet<_> = ctx.whitelisted_items().collect();

        let whitelisted_and_blacklisted_items: ItemSet = whitelisted_items.iter()
            .cloned()
            .flat_map(|i| {
                let mut reachable = vec![i];
                i.trace(ctx, &mut |s, _| {
                    reachable.push(s);
                }, &());
                reachable
            })
            .collect();

        for item in whitelisted_and_blacklisted_items {
            dependencies.entry(item).or_insert(vec![]);

            {
                // We reverse our natural IR graph edges to find dependencies
                // between nodes.
                item.trace(ctx, &mut |sub_item: ItemId, edge_kind| {
                    if Self::consider_edge(edge_kind) {
                        dependencies.entry(sub_item)
                            .or_insert(vec![])
                            .push(item);
                    }
                }, &());
            }
        }

        CantDeriveDebugAnalysis {
            ctx: ctx,
            cant_derive_debug: cant_derive_debug,
            dependencies: dependencies,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        self.ctx.whitelisted_items().collect()
    }

    fn constrain(&mut self, id: ItemId) -> bool {
        if self.cant_derive_debug.contains(&id) {
            return false;
        }

        let item = self.ctx.resolve_item(id);
        let ty = match item.as_type() {
            None => return false,
            Some(ty) => ty
        };

        match *ty.kind() {
            // handle the simple case
            // These can derive debug without further information
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
                return false;
            },
            TypeKind::Opaque => {
                if ty.layout(self.ctx)
                    .map_or(true, |l| l.opaque().can_trivially_derive_debug(self.ctx, ())) {
                        return false;
                    } else {
                        return self.insert(id);
                    }
            },
            TypeKind::Array(t, len) => {
                if len <= RUST_DERIVE_IN_ARRAY_LIMIT {
                    if self.cant_derive_debug.contains(&t) {
                        return self.insert(id);
                    }
                    return false;
                } else {
                    return self.insert(id);
                }
            },
            TypeKind::ResolvedTypeRef(t) |
            TypeKind::TemplateAlias(t, _) |
            TypeKind::Alias(t) => {
                if self.cant_derive_debug.contains(&t) {
                    return self.insert(id);
                }
                return false;
            },
            TypeKind::Comp(ref info) => {
                if info.has_non_type_template_params() {
                    if ty.layout(self.ctx).map_or(true,
                                                  |l| l.opaque().can_trivially_derive_debug(self.ctx, ())) {
                        return false;
                    } else {
                        return self.insert(id);
                    }
                }
                if info.kind() == CompKind::Union {
                    if self.ctx.options().unstable_rust {
                        return self.insert(id);
                    }

                    if ty.layout(self.ctx).map_or(true,
                                                  |l| l.opaque().can_trivially_derive_debug(self.ctx, ())) {
                        return false;
                    } else {
                        return self.insert(id);
                    }
                }
                let bases_cant_derive = info.base_members()
                    .iter()
                    .any(|base| self.cant_derive_debug.contains(&base.ty));
                if bases_cant_derive {
                    return self.insert(id);
                }
                let fields_cant_derive = info.fields()
                    .iter()
                    .any(|f| {
                        match f {
                            &Field::DataMember(ref data) => self.cant_derive_debug.contains(&data.ty()),
                            &Field::Bitfields(ref bfu) => bfu.bitfields()
                                .iter().any(|b| {
                                    self.cant_derive_debug.contains(&b.ty())
                                })
                        }
                    });
                if fields_cant_derive {
                    return self.insert(id);
                }
                false
            },
            TypeKind::Pointer(inner) => {
                let inner_type = self.ctx.resolve_type(inner);
                if let TypeKind::Function(ref sig) =
                    *inner_type.canonical_type(self.ctx).kind() {
                        if sig.can_trivially_derive_debug(&self.ctx, ()) {
                            return false;
                        } else {
                            return self.insert(id);
                        }
                    }
                false
            },
            TypeKind::TemplateInstantiation(ref template) => {
                let args_cant_derive = template.template_arguments()
                    .iter()
                    .any(|arg| self.cant_derive_debug.contains(&arg));
                if args_cant_derive {
                    return self.insert(id);
                }
                let ty_cant_derive = template.template_definition()
                    .into_resolver()
                    .through_type_refs()
                    .through_type_aliases()
                    .resolve(self.ctx)
                    .as_type()
                    .expect("Instantiations of a non-type?")
                    .as_comp()
                    .and_then(|c| {
                        // For non-type template parameters, we generate an opaque
                        // blob, and in this case the instantiation has a better
                        // idea of the layout than the definition does.
                        if c.has_non_type_template_params() {
                            let opaque = ty.layout(self.ctx)
                                .or_else(|| self.ctx.resolve_type(template.template_definition()).layout(self.ctx))
                                .unwrap_or(Layout::zero())
                                .opaque();
                            Some(!opaque.can_trivially_derive_debug(&self.ctx, ()))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| self.cant_derive_debug.contains(&template.template_definition()));
                if ty_cant_derive {
                    return self.insert(id);
                }
                false
            },
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

impl<'ctx, 'gen> From<CantDeriveDebugAnalysis<'ctx, 'gen>> for HashSet<ItemId> {
    fn from(analysis: CantDeriveDebugAnalysis<'ctx, 'gen>) -> Self {
        analysis.cant_derive_debug
    }
}
