//! Determining which types cannot be passed by value.

use super::{generate_dependencies, ConstrainResult, MonotoneFramework};
use crate::ir::comp::Field;
use crate::ir::comp::FieldMethods;
use crate::ir::context::{BindgenContext, ItemId};
use crate::ir::traversal::EdgeKind;
use crate::ir::ty::TypeKind;
use crate::{HashMap, HashSet};

/// An analysis that finds each IR item which is a type which cannot be passed by value in Rust.
///
/// This is defined as follows:
///
/// * If T is float or complex float with size not 32 or 64 bit (twice that for complex), it cannot
///   be passed by value.
/// * If T is a type alias, a templated alias or an indirection to another type,
///   it matches if the type T refers to does
/// * If T is a compound type, it matches if any of base memter or field does.
/// * If T is an instantiation of an abstract template definition, T matches if any of the template
///   arguments or template definition do.
///
/// A more advanced implementation might be aware of which registers arguments will actually end up
/// in and permit some signatures this rejects on a platform-specific basis.
#[derive(Debug, Clone)]
pub struct NeverByValue<'ctx> {
    ctx: &'ctx BindgenContext,

    // The incremental result of this analysis's computation. Everything in this
    // set has float.
    never_by_value: HashSet<ItemId>,

    // Dependencies saying that if a key ItemId has been inserted into the
    // `never_by_value` set, then each of the ids in Vec<ItemId> need to be
    // considered again.
    //
    // This is a subset of the natural IR graph with reversed edges, where we
    // only include the edges from the IR graph that can affect whether a type
    // has float or not.
    dependencies: HashMap<ItemId, Vec<ItemId>>,
}

impl<'ctx> NeverByValue<'ctx> {
    fn consider_edge(kind: EdgeKind) -> bool {
        match kind {
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

    fn insert<Id: Into<ItemId>>(&mut self, id: Id) -> ConstrainResult {
        let id = id.into();
        trace!("inserting {id:?} into the never_by_value set");

        let was_not_already_in_set = self.never_by_value.insert(id);
        assert!(
            was_not_already_in_set,
            "We shouldn't try and insert {:?} twice because if it was \
             already in the set, `constrain` should have exited early.",
            id
        );

        ConstrainResult::Changed
    }
}

impl<'ctx> MonotoneFramework for NeverByValue<'ctx> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext;
    type Output = HashSet<ItemId>;

    fn new(ctx: &'ctx BindgenContext) -> Self {
        let never_by_value = HashSet::default();
        let dependencies = generate_dependencies(ctx, Self::consider_edge);

        NeverByValue {
            ctx,
            never_by_value,
            dependencies,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        self.ctx.allowlisted_items().iter().cloned().collect()
    }

    fn constrain(&mut self, id: ItemId) -> ConstrainResult {
        trace!("constrain: {id:?}");

        if self.never_by_value.contains(&id) {
            trace!("    already in set");
            return ConstrainResult::Same;
        }

        let item = self.ctx.resolve_item(id);
        if let Some(ty) = item.kind().as_type() {
            match *ty.kind() {
                TypeKind::Void |
                TypeKind::NullPtr |
                TypeKind::Int(..) |
                TypeKind::Function(..) |
                TypeKind::Enum(..) |
                TypeKind::Reference(..) |
                TypeKind::TypeParam |
                TypeKind::Opaque |
                TypeKind::Pointer(..) |
                TypeKind::UnresolvedTypeRef(..) |
                TypeKind::ObjCInterface(..) |
                TypeKind::ObjCId |
                TypeKind::ObjCSel |
                TypeKind::BlockPointer(_) => {
                    trace!("    simple type that is not float");
                    ConstrainResult::Same
                }

                TypeKind::Float(..) | TypeKind::Complex(..) => {
                    if let Some(layout) = ty.layout(self.ctx) {
                        match (ty.kind(), layout.size) {
                            (TypeKind::Float(..), 4 | 8) |
                            (TypeKind::Complex(..), 8 | 16) => {
                                trace!("    skipped f32 or f64");
                                ConstrainResult::Same
                            }
                            _ => {
                                trace!(
                                    "    extended float size {}",
                                    layout.size
                                );
                                self.insert(id)
                            }
                        }
                    } else {
                        // This case comes up with macro constants and doesn't seem relevant.
                        trace!("    unknown float");
                        ConstrainResult::Same
                    }
                }

                TypeKind::Alias(t) |
                TypeKind::Array(t, _) |
                TypeKind::ResolvedTypeRef(t) |
                TypeKind::TemplateAlias(t, _) |
                TypeKind::Vector(t, _) => {
                    if self.never_by_value.contains(&t.into()) {
                        trace!(
                            "    contains/aliases matching type, so matches"
                        );
                        self.insert(id)
                    } else {
                        trace!("    does not contain/alias matching type");
                        ConstrainResult::Same
                    }
                }

                TypeKind::Comp(ref info) => {
                    let bases_have = info.base_members().iter().any(|base| {
                        self.never_by_value.contains(&base.ty.into())
                    });
                    if bases_have {
                        trace!("    bases have float, so we also have");
                        return self.insert(id);
                    }
                    let fields_have = info.fields().iter().any(|f| match *f {
                        Field::DataMember(ref data) => {
                            self.never_by_value.contains(&data.ty().into())
                        }
                        Field::Bitfields(ref bfu) => {
                            bfu.bitfields().iter().any(|b| {
                                self.never_by_value.contains(&b.ty().into())
                            })
                        }
                    });
                    if fields_have {
                        trace!("    fields have float, so we also have");
                        return self.insert(id);
                    }

                    trace!("    comp doesn't have float");
                    ConstrainResult::Same
                }

                TypeKind::TemplateInstantiation(ref template) => {
                    let args_have = template
                        .template_arguments()
                        .iter()
                        .any(|arg| self.never_by_value.contains(&arg.into()));
                    if args_have {
                        trace!("    template args match, so instantiation also matches");
                        return self.insert(id);
                    }

                    let def_has = self
                        .never_by_value
                        .contains(&template.template_definition().into());
                    if def_has {
                        trace!("    template definition has float, so instantiation also has");
                        return self.insert(id);
                    }

                    trace!("    template instantiation does not match");
                    ConstrainResult::Same
                }
            }
        } else {
            trace!("    not a type; skipped");
            ConstrainResult::Same
        }
    }

    fn each_depending_on<F>(&self, id: ItemId, mut f: F)
    where
        F: FnMut(ItemId),
    {
        if let Some(edges) = self.dependencies.get(&id) {
            for item in edges {
                trace!("enqueue {item:?} into worklist");
                f(*item);
            }
        }
    }
}

impl<'ctx> From<NeverByValue<'ctx>> for HashSet<ItemId> {
    fn from(analysis: NeverByValue<'ctx>) -> Self {
        analysis.never_by_value
    }
}
