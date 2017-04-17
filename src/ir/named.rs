//! Discover which template type parameters are actually used.
//!
//! ### Why do we care?
//!
//! C++ allows ignoring template parameters, while Rust does not. Usually we can
//! blindly stick a `PhantomData<T>` inside a generic Rust struct to make up for
//! this. That doesn't work for templated type aliases, however:
//!
//! ```C++
//! template <typename T>
//! using Fml = int;
//! ```
//!
//! If we generate the naive Rust code for this alias, we get:
//!
//! ```ignore
//! pub type Fml<T> = ::std::os::raw::int;
//! ```
//!
//! And this is rejected by `rustc` due to the unused type parameter.
//!
//! (Aside: in these simple cases, `libclang` will often just give us the
//! aliased type directly, and we will never even know we were dealing with
//! aliases, let alone templated aliases. It's the more convoluted scenarios
//! where we get to have some fun...)
//!
//! For such problematic template aliases, we could generate a tuple whose
//! second member is a `PhantomData<T>`. Or, if we wanted to go the extra mile,
//! we could even generate some smarter wrapper that implements `Deref`,
//! `DerefMut`, `From`, `Into`, `AsRef`, and `AsMut` to the actually aliased
//! type. However, this is still lackluster:
//!
//! 1. Even with a billion conversion-trait implementations, using the generated
//!    bindings is rather un-ergonomic.
//! 2. With either of these solutions, we need to keep track of which aliases
//!    we've transformed like this in order to generate correct uses of the
//!    wrapped type.
//!
//! Given that we have to properly track which template parameters ended up used
//! for (2), we might as well leverage that information to make ergonomic
//! bindings that don't contain any unused type parameters at all, and
//! completely avoid the pain of (1).
//!
//! ### How do we determine which template parameters are used?
//!
//! Determining which template parameters are actually used is a trickier
//! problem than it might seem at a glance. On the one hand, trivial uses are
//! easy to detect:
//!
//! ```C++
//! template <typename T>
//! class Foo {
//!     T trivial_use_of_t;
//! };
//! ```
//!
//! It gets harder when determining if one template parameter is used depends on
//! determining if another template parameter is used. In this example, whether
//! `U` is used depends on whether `T` is used.
//!
//! ```C++
//! template <typename T>
//! class DoesntUseT {
//!     int x;
//! };
//!
//! template <typename U>
//! class Fml {
//!     DoesntUseT<U> lololol;
//! };
//! ```
//!
//! We can express the set of used template parameters as a constraint solving
//! problem (where the set of template parameters used by a given IR item is the
//! union of its sub-item's used template parameters) and iterate to a
//! fixed-point.
//!
//! We use the "monotone framework" for this fix-point analysis where our
//! lattice is the mapping from each IR item to the powerset of the template
//! parameters that appear in the input C++ header, our join function is set
//! union, and we use the `ir::traversal::Trace` trait to implement the
//! work-list optimization so we don't have to revisit every node in the graph
//! when for every iteration towards the fix-point.
//!
//! A lattice is a set with a partial ordering between elements, where there is
//! a single least upper bound and a single greatest least bound for every
//! subset. We are dealing with finite lattices, which means that it has a
//! finite number of elements, and it follows that there exists a single top and
//! a single bottom member of the lattice. For example, the power set of a
//! finite set forms a finite lattice where partial ordering is defined by set
//! inclusion, that is `a <= b` if `a` is a subset of `b`. Here is the finite
//! lattice constructed from the set {0,1,2}:
//!
//! ```text
//!                    .----- Top = {0,1,2} -----.
//!                   /            |              \
//!                  /             |               \
//!                 /              |                \
//!              {0,1} -------.  {0,2}  .--------- {1,2}
//!                |           \ /   \ /             |
//!                |            /     \              |
//!                |           / \   / \             |
//!               {0} --------'   {1}   `---------- {2}
//!                 \              |                /
//!                  \             |               /
//!                   \            |              /
//!                    `------ Bottom = {} ------'
//! ```
//!
//! A monotone function `f` is a function where if `x <= y`, then it holds that
//! `f(x) <= f(y)`. It should be clear that running a monotone function to a
//! fix-point on a finite lattice will always terminate: `f` can only "move"
//! along the lattice in a single direction, and therefore can only either find
//! a fix-point in the middle of the lattice or continue to the top or bottom
//! depending if it is ascending or descending the lattice respectively.
//!
//! For our analysis, we are collecting the set of template parameters used by
//! any given IR node. The set of template parameters appearing in the program
//! is finite. Our lattice is their powerset. We start at the bottom element,
//! the empty set. Our analysis only adds members to the set of used template
//! parameters, never removes them, so it is monotone, and therefore iteration
//! to a fix-point will terminate.
//!
//! For a deeper introduction to the general form of this kind of analysis, see
//! [Static Program Analysis by Anders Møller and Michael I. Schwartzbach][spa].
//!
//! [spa]: https://cs.au.dk/~amoeller/spa/spa.pdf

use super::context::{BindgenContext, ItemId};
use super::item::{Item, ItemSet};
use super::template::{AsNamed, TemplateInstantiation};
use super::traversal::{EdgeKind, Trace};
use super::ty::{TemplateDeclaration, TypeKind};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// An analysis in the monotone framework.
///
/// Implementors of this trait must maintain the following two invariants:
///
/// 1. The concrete data must be a member of a finite-height lattice.
/// 2. The concrete `constrain` method must be monotone: that is,
///    if `x <= y`, then `constrain(x) <= constrain(y)`.
///
/// If these invariants do not hold, iteration to a fix-point might never
/// complete.
///
/// For a simple example analysis, see the `ReachableFrom` type in the `tests`
/// module below.
pub trait MonotoneFramework: Sized + fmt::Debug {
    /// The type of node in our dependency graph.
    ///
    /// This is just generic (and not `ItemId`) so that we can easily unit test
    /// without constructing real `Item`s and their `ItemId`s.
    type Node: Copy;

    /// Any extra data that is needed during computation.
    ///
    /// Again, this is just generic (and not `&BindgenContext`) so that we can
    /// easily unit test without constructing real `BindgenContext`s full of
    /// real `Item`s and real `ItemId`s.
    type Extra: Sized;

    /// The final output of this analysis. Once we have reached a fix-point, we
    /// convert `self` into this type, and return it as the final result of the
    /// analysis.
    type Output: From<Self> + fmt::Debug;

    /// Construct a new instance of this analysis.
    fn new(extra: Self::Extra) -> Self;

    /// Get the initial set of nodes from which to start the analysis. Unless
    /// you are sure of some domain-specific knowledge, this should be the
    /// complete set of nodes.
    fn initial_worklist(&self) -> Vec<Self::Node>;

    /// Update the analysis for the given node.
    ///
    /// If this results in changing our internal state (ie, we discovered that
    /// we have not reached a fix-point and iteration should continue), return
    /// `true`. Otherwise, return `false`. When `constrain` returns false for
    /// all nodes in the set, we have reached a fix-point and the analysis is
    /// complete.
    fn constrain(&mut self, node: Self::Node) -> bool;

    /// For each node `d` that depends on the given `node`'s current answer when
    /// running `constrain(d)`, call `f(d)`. This informs us which new nodes to
    /// queue up in the worklist when `constrain(node)` reports updated
    /// information.
    fn each_depending_on<F>(&self, node: Self::Node, f: F)
        where F: FnMut(Self::Node);
}

/// Run an analysis in the monotone framework.
pub fn analyze<Analysis>(extra: Analysis::Extra) -> Analysis::Output
    where Analysis: MonotoneFramework,
{
    let mut analysis = Analysis::new(extra);
    let mut worklist = analysis.initial_worklist();

    while let Some(node) = worklist.pop() {
        if analysis.constrain(node) {
            analysis.each_depending_on(node, |needs_work| {
                worklist.push(needs_work);
            });
        }
    }

    analysis.into()
}

/// An analysis that finds for each IR item its set of template parameters that
/// it uses.
///
/// We use the following monotone constraint function:
///
/// ```ignore
/// template_param_usage(v) =
///     self_template_param_usage(v) union
///     template_param_usage(w_0) union
///     template_param_usage(w_1) union
///     ...
///     template_param_usage(w_n)
/// ```
///
/// Where `v` has direct edges in the IR graph to each of `w_0`, `w_1`,
/// ..., `w_n` (for example, if `v` were a struct type and `x` and `y`
/// were the types of two of `v`'s fields). We ignore certain edges, such
/// as edges from a template declaration to its template parameters'
/// definitions for this analysis. If we didn't, then we would mistakenly
/// determine that ever template parameter is always used.
///
/// Finally, `self_template_param_usage` is defined with the following cases:
///
/// * If `T` is a template parameter:
///
/// ```ignore
/// self_template_param_usage(T) = { T }
/// ```
///
/// * If `inst` is a template instantiation, `inst.args` are the template
///   instantiation's template arguments, and `inst.decl` is the template
///   declaration being instantiated:
///
/// ```ignore
/// self_template_param_usage(inst) =
///     { T: for T in inst.args, if T in template_param_usage(inst.decl) }
/// ```
///
/// * And for all other IR items, the result is the empty set:
///
/// ```ignore
/// self_template_param_usage(_) = { }
/// ```
#[derive(Debug, Clone)]
pub struct UsedTemplateParameters<'ctx, 'gen>
    where 'gen: 'ctx,
{
    ctx: &'ctx BindgenContext<'gen>,

    // The Option is only there for temporary moves out of the hash map. See the
    // comments in `UsedTemplateParameters::constrain` below.
    used: HashMap<ItemId, Option<ItemSet>>,

    dependencies: HashMap<ItemId, Vec<ItemId>>,

    whitelisted_items: HashSet<ItemId>,
}

impl<'ctx, 'gen> UsedTemplateParameters<'ctx, 'gen> {
    fn consider_edge(kind: EdgeKind) -> bool {
        match kind {
            // For each of these kinds of edges, if the referent uses a template
            // parameter, then it should be considered that the origin of the
            // edge also uses the template parameter.
            EdgeKind::TemplateArgument |
            EdgeKind::BaseMember |
            EdgeKind::Field |
            EdgeKind::Constructor |
            EdgeKind::VarType |
            EdgeKind::FunctionReturn |
            EdgeKind::FunctionParameter |
            EdgeKind::TypeReference => true,

            // An inner var or type using a template parameter is orthogonal
            // from whether we use it. See template-param-usage-{6,11}.hpp.
            EdgeKind::InnerVar | EdgeKind::InnerType => false,

            // We can't emit machine code for new monomorphizations of class
            // templates' methods (and don't detect explicit instantiations) so
            // we must ignore template parameters that are only used by
            // methods. This doesn't apply to a function type's return or
            // parameter types, however, because of type aliases of function
            // pointers that use template parameters, eg
            // tests/headers/struct_with_typedef_template_arg.hpp
            EdgeKind::Method => false,

            // If we considered these edges, we would end up mistakenly claiming
            // that every template parameter always used.
            EdgeKind::TemplateDeclaration |
            EdgeKind::TemplateParameterDefinition => false,

            // Since we have to be careful about which edges we consider for
            // this analysis to be correct, we ignore generic edges. We also
            // avoid a `_` wild card to force authors of new edge kinds to
            // determine whether they need to be considered by this analysis.
            EdgeKind::Generic => false,
        }
    }

    fn take_this_id_usage_set(&mut self, this_id: ItemId) -> ItemSet {
        self.used
            .get_mut(&this_id)
            .expect("Should have a set of used template params for every item \
                     id")
            .take()
            .expect("Should maintain the invariant that all used template param \
                     sets are `Some` upon entry of `constrain`")
    }

    /// We say that blacklisted items use all of their template parameters. The
    /// blacklisted type is most likely implemented explicitly by the user,
    /// since it won't be in the generated bindings, and we don't know exactly
    /// what they'll to with template parameters, but we can push the issue down
    /// the line to them.
    fn constrain_instantiation_of_blacklisted_template(&self,
                                                       used_by_this_id: &mut ItemSet,
                                                       instantiation: &TemplateInstantiation) {
        debug!("    instantiation of blacklisted template, uses all template \
                arguments");

        let args = instantiation.template_arguments()
            .iter()
            .filter_map(|a| a.as_named(self.ctx, &()));
        used_by_this_id.extend(args);
    }

    /// A template instantiation's concrete template argument is only used if
    /// the template definition uses the corresponding template parameter.
    fn constrain_instantiation(&self,
                               used_by_this_id: &mut ItemSet,
                               instantiation: &TemplateInstantiation) {
        debug!("    template instantiation");

        let decl = self.ctx.resolve_type(instantiation.template_definition());
        let args = instantiation.template_arguments();

        let params = decl.self_template_params(self.ctx)
            .unwrap_or(vec![]);

        let used_by_def = self.used[&instantiation.template_definition()]
            .as_ref()
            .unwrap();

        for (arg, param) in args.iter().zip(params.iter()) {
            debug!("      instantiation's argument {:?} is used if definition's \
                    parameter {:?} is used",
                   arg,
                   param);

            if used_by_def.contains(param) {
                debug!("        param is used by template definition");

                let arg = arg.into_resolver()
                    .through_type_refs()
                    .through_type_aliases()
                    .resolve(self.ctx);
                if let Some(named) = arg.as_named(self.ctx, &()) {
                    debug!("        arg is a type parameter, marking used");
                    used_by_this_id.insert(named);
                }
            }
        }
    }

    /// The join operation on our lattice: the set union of all of this id's
    /// successors.
    fn constrain_join(&self, used_by_this_id: &mut ItemSet, item: &Item) {
        debug!("    other item: join with successors' usage");

        item.trace(self.ctx, &mut |sub_id, edge_kind| {
            // Ignore ourselves, since union with ourself is a
            // no-op. Ignore edges that aren't relevant to the
            // analysis. Ignore edges to blacklisted items.
            if sub_id == item.id() ||
                !Self::consider_edge(edge_kind) ||
                !self.whitelisted_items.contains(&sub_id) {
                    return;
                }

            let used_by_sub_id = self.used[&sub_id]
                .as_ref()
                .expect("Because sub_id != id, and all used template \
                         param sets other than id's are `Some`, \
                         sub_id's used template param set should be \
                         `Some`")
                        .iter()
                        .cloned();

            debug!("      union with {:?}'s usage: {:?}",
                   sub_id,
                   used_by_sub_id.clone().collect::<Vec<_>>());

            used_by_this_id.extend(used_by_sub_id);
        }, &());
    }
}

impl<'ctx, 'gen> MonotoneFramework for UsedTemplateParameters<'ctx, 'gen> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext<'gen>;
    type Output = HashMap<ItemId, ItemSet>;

    fn new(ctx: &'ctx BindgenContext<'gen>)
           -> UsedTemplateParameters<'ctx, 'gen> {
        let mut used = HashMap::new();
        let mut dependencies = HashMap::new();
        let whitelisted_items: HashSet<_> = ctx.whitelisted_items().collect();

        for item in whitelisted_items.iter().cloned() {
            dependencies.entry(item).or_insert(vec![]);
            used.entry(item).or_insert(Some(ItemSet::new()));

            {
                // We reverse our natural IR graph edges to find dependencies
                // between nodes.
                item.trace(ctx, &mut |sub_item, _| {
                    used.entry(sub_item).or_insert(Some(ItemSet::new()));

                    // We won't be generating code for items that aren't
                    // whitelisted, so don't bother keeping track of their
                    // template parameters. But isn't whitelisting the
                    // transitive closure of reachable items from the explicitly
                    // whitelisted items? Usually! The exception is explicitly
                    // blacklisted items.
                    if !whitelisted_items.contains(&sub_item) {
                        return;
                    }

                    dependencies.entry(sub_item)
                        .or_insert(vec![])
                        .push(item);
                }, &());
            }

            // Additionally, whether a template instantiation's template
            // arguments are used depends on whether the template declaration's
            // generic template parameters are used.
            ctx.resolve_item(item)
                .as_type()
                .map(|ty| match ty.kind() {
                    &TypeKind::TemplateInstantiation(ref inst) => {
                        let decl = ctx.resolve_type(inst.template_definition());
                        let args = inst.template_arguments();

                        // Although template definitions should always have
                        // template parameters, there is a single exception:
                        // opaque templates. Hence the unwrap_or.
                        let params = decl.self_template_params(ctx)
                            .unwrap_or(vec![]);

                        for (arg, param) in args.iter().zip(params.iter()) {
                            used.entry(*arg).or_insert(Some(ItemSet::new()));
                            used.entry(*param).or_insert(Some(ItemSet::new()));

                            dependencies.entry(*arg)
                                .or_insert(vec![])
                                .push(*param);
                        }
                    }
                    _ => {}
                });
        }

        if cfg!(feature = "testing_only_extra_assertions") {
            // Invariant: The `used` map has an entry for every whitelisted
            // item, as well as all explicitly blacklisted items that are
            // reachable from whitelisted items.
            //
            // (This is so that every item we call `constrain` on is guaranteed
            // to have a set of template parameters, and we can allow
            // blacklisted templates to use all of their parameters).
            for item in whitelisted_items.iter() {
                extra_assert!(used.contains_key(item));
                item.trace(ctx, &mut |sub_item, _| {
                    extra_assert!(used.contains_key(&sub_item));
                }, &())
            }

            // Invariant: the `dependencies` map has an entry for every
            // whitelisted item.
            for item in whitelisted_items.iter() {
                extra_assert!(dependencies.contains_key(item));
            }
        }

        UsedTemplateParameters {
            ctx: ctx,
            used: used,
            dependencies: dependencies,
            whitelisted_items: whitelisted_items,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        self.ctx.whitelisted_items().collect()
    }

    fn constrain(&mut self, id: ItemId) -> bool {
        // Invariant: all hash map entries' values are `Some` upon entering and
        // exiting this method.
        extra_assert!(self.used.values().all(|v| v.is_some()));

        // Take the set for this id out of the hash map while we mutate it based
        // on other hash map entries. We *must* put it back into the hash map at
        // the end of this method. This allows us to side-step HashMap's lack of
        // an analog to slice::split_at_mut.
        let mut used_by_this_id = self.take_this_id_usage_set(id);

        debug!("constrain {:?}", id);
        debug!("  initially, used set is {:?}", used_by_this_id);

        let original_len = used_by_this_id.len();

        let item = self.ctx.resolve_item(id);
        let ty_kind = item.as_type().map(|ty| ty.kind());
        match ty_kind {
            // Named template type parameters trivially use themselves.
            Some(&TypeKind::Named) => {
                debug!("    named type, trivially uses itself");
                used_by_this_id.insert(id);
            }
            // Template instantiations only use their template arguments if the
            // template definition uses the corresponding template parameter.
            Some(&TypeKind::TemplateInstantiation(ref inst)) => {
                if self.whitelisted_items.contains(&inst.template_definition()) {
                    self.constrain_instantiation(&mut used_by_this_id, inst);
                } else {
                    self.constrain_instantiation_of_blacklisted_template(&mut used_by_this_id,
                                                                         inst);
                }
            }
            // Otherwise, add the union of each of its referent item's template
            // parameter usage.
            _ => self.constrain_join(&mut used_by_this_id, item),
        }

        debug!("  finally, used set is {:?}", used_by_this_id);

        let new_len = used_by_this_id.len();
        assert!(new_len >= original_len,
                "This is the property that ensures this function is monotone -- \
                 if it doesn't hold, the analysis might never terminate!");

        // Put the set back in the hash map and restore our invariant.
        debug_assert!(self.used[&id].is_none());
        self.used.insert(id, Some(used_by_this_id));
        extra_assert!(self.used.values().all(|v| v.is_some()));

        new_len != original_len
    }

    fn each_depending_on<F>(&self, item: ItemId, mut f: F)
        where F: FnMut(ItemId),
    {
        if let Some(edges) = self.dependencies.get(&item) {
            for item in edges {
                debug!("enqueue {:?} into worklist", item);
                f(*item);
            }
        }
    }
}

impl<'ctx, 'gen> From<UsedTemplateParameters<'ctx, 'gen>>
    for HashMap<ItemId, ItemSet> {
    fn from(used_templ_params: UsedTemplateParameters<'ctx, 'gen>) -> Self {
        used_templ_params.used
            .into_iter()
            .map(|(k, v)| (k, v.unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    // Here we find the set of nodes that are reachable from any given
    // node. This is a lattice mapping nodes to subsets of all nodes. Our join
    // function is set union.
    //
    // This is our test graph:
    //
    //     +---+                    +---+
    //     |   |                    |   |
    //     | 1 |               .----| 2 |
    //     |   |               |    |   |
    //     +---+               |    +---+
    //       |                 |      ^
    //       |                 |      |
    //       |      +---+      '------'
    //       '----->|   |
    //              | 3 |
    //       .------|   |------.
    //       |      +---+      |
    //       |        ^        |
    //       v        |        v
    //     +---+      |      +---+    +---+
    //     |   |      |      |   |    |   |
    //     | 4 |      |      | 5 |--->| 6 |
    //     |   |      |      |   |    |   |
    //     +---+      |      +---+    +---+
    //       |        |        |        |
    //       |        |        |        v
    //       |      +---+      |      +---+
    //       |      |   |      |      |   |
    //       '----->| 7 |<-----'      | 8 |
    //              |   |             |   |
    //              +---+             +---+
    //
    // And here is the mapping from a node to the set of nodes that are
    // reachable from it within the test graph:
    //
    //     1: {3,4,5,6,7,8}
    //     2: {2}
    //     3: {3,4,5,6,7,8}
    //     4: {3,4,5,6,7,8}
    //     5: {3,4,5,6,7,8}
    //     6: {8}
    //     7: {3,4,5,6,7,8}
    //     8: {}

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    struct Node(usize);

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct Graph(HashMap<Node, Vec<Node>>);

    impl Graph {
        fn make_test_graph() -> Graph {
            let mut g = Graph::default();
            g.0.insert(Node(1), vec![Node(3)]);
            g.0.insert(Node(2), vec![Node(2)]);
            g.0.insert(Node(3), vec![Node(4), Node(5)]);
            g.0.insert(Node(4), vec![Node(7)]);
            g.0.insert(Node(5), vec![Node(6), Node(7)]);
            g.0.insert(Node(6), vec![Node(8)]);
            g.0.insert(Node(7), vec![Node(3)]);
            g.0.insert(Node(8), vec![]);
            g
        }

        fn reverse(&self) -> Graph {
            let mut reversed = Graph::default();
            for (node, edges) in self.0.iter() {
                reversed.0.entry(*node).or_insert(vec![]);
                for referent in edges.iter() {
                    reversed.0.entry(*referent).or_insert(vec![]).push(*node);
                }
            }
            reversed
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct ReachableFrom<'a> {
        reachable: HashMap<Node, HashSet<Node>>,
        graph: &'a Graph,
        reversed: Graph,
    }

    impl<'a> MonotoneFramework for ReachableFrom<'a> {
        type Node = Node;
        type Extra = &'a Graph;
        type Output = HashMap<Node, HashSet<Node>>;

        fn new(graph: &'a Graph) -> ReachableFrom {
            let reversed = graph.reverse();
            ReachableFrom {
                reachable: Default::default(),
                graph: graph,
                reversed: reversed,
            }
        }

        fn initial_worklist(&self) -> Vec<Node> {
            self.graph.0.keys().cloned().collect()
        }

        fn constrain(&mut self, node: Node) -> bool {
            // The set of nodes reachable from a node `x` is
            //
            //     reachable(x) = s_0 U s_1 U ... U reachable(s_0) U reachable(s_1) U ...
            //
            // where there exist edges from `x` to each of `s_0, s_1, ...`.
            //
            // Yes, what follows is a **terribly** inefficient set union
            // implementation. Don't copy this code outside of this test!

            let original_size =
                self.reachable.entry(node).or_insert(HashSet::new()).len();

            for sub_node in self.graph.0[&node].iter() {
                self.reachable.get_mut(&node).unwrap().insert(*sub_node);

                let sub_reachable = self.reachable
                    .entry(*sub_node)
                    .or_insert(HashSet::new())
                    .clone();

                for transitive in sub_reachable {
                    self.reachable.get_mut(&node).unwrap().insert(transitive);
                }
            }

            let new_size = self.reachable[&node].len();
            original_size != new_size
        }

        fn each_depending_on<F>(&self, node: Node, mut f: F)
            where F: FnMut(Node),
        {
            for dep in self.reversed.0[&node].iter() {
                f(*dep);
            }
        }
    }

    impl<'a> From<ReachableFrom<'a>> for HashMap<Node, HashSet<Node>> {
        fn from(reachable: ReachableFrom<'a>) -> Self {
            reachable.reachable
        }
    }

    #[test]
    fn monotone() {
        let g = Graph::make_test_graph();
        let reachable = analyze::<ReachableFrom>(&g);
        println!("reachable = {:#?}", reachable);

        fn nodes<A>(nodes: A) -> HashSet<Node>
            where A: AsRef<[usize]>,
        {
            nodes.as_ref().iter().cloned().map(Node).collect()
        }

        let mut expected = HashMap::new();
        expected.insert(Node(1), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(2), nodes([2]));
        expected.insert(Node(3), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(4), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(5), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(6), nodes([8]));
        expected.insert(Node(7), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(8), nodes([]));
        println!("expected = {:#?}", expected);

        assert_eq!(reachable, expected);
    }
}
