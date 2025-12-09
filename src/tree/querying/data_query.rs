use crate::tree::node::MaterializedNode;
use crate::tree::querying::presentation::{First, Grouped, QueryPresentation};
use crate::tree::scopes::ScopeDefinition;
use crate::tree::traits::NodeRepository;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};
use std::marker::PhantomData;

trait QueryNodeRepo {
    fn query(node_repo: &impl NodeRepository) -> Self;
}

macro_rules! impl_query_node_repo_for_tuples {
    () => {};

    ($head:ident $(, $tail:ident)*) => {
        impl<$head, $($tail),*> QueryNodeRepo for ($head, $($tail),*)
        where
            $head: QueryNodeRepo,
            $($tail: QueryNodeRepo),*
        {
            fn query(node_repo: &impl NodeRepository) -> Self {
                (
                    $head::query(node_repo),
                    $($tail::query(node_repo)),*
                )
            }
        }

        impl_query_node_repo_for_tuples!($($tail),*);
    };
}
impl_query_node_repo_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

struct QueryNodesInScope<Scope: ScopeDefinition, NodeType, Quantity> {
    pub result: Quantity,
    scope: PhantomData<Scope>,
    node_type: PhantomData<NodeType>,
}

impl<
    Scope: ScopeDefinition,
    NodeType: Clone + 'static,
    Quantity: QueryPresentation<Vec<MaterializedNode<NodeType>>>,
> QueryNodeRepo for QueryNodesInScope<Scope, NodeType, Quantity>
{
    fn query(node_repo: &impl NodeRepository) -> Self {
        let a = node_repo
            .get_nodes_in_scope::<NodeType>(Scope::layer())
            .unwrap_or_default();

        QueryNodesInScope {
            result: Quantity::present(a),
            scope: PhantomData,
            node_type: PhantomData,
        }
    }
}

struct QueryGroupedNodes<Scope: ScopeDefinition, NodeType, Quantity> {
    pub result: Quantity,
    _scope: PhantomData<Scope>,
    _node: PhantomData<NodeType>,
}

impl<
    Scope: ScopeDefinition,
    NodeType: Clone + 'static,
    Quantity: QueryPresentation<Vec<Vec<MaterializedNode<NodeType>>>>,
> QueryNodeRepo for QueryGroupedNodes<Scope, NodeType, Quantity>
{
    fn query(node_repo: &impl NodeRepository) -> Self {
        let a = node_repo
            .get_nodes_for_scope_per_top_level_element::<NodeType>(Scope::layer())
            .unwrap_or_default();

        QueryGroupedNodes {
            result: Quantity::present(a),
            _scope: Default::default(),
            _node: Default::default(),
        }
    }
}

// For show off

trait TheRuleTrait {
    type Query: QueryNodeRepo;

    fn check_erased(&'_ self, board: Self::Query) -> bool;
}

struct __RuleImplementation1;

impl TheRuleTrait for __RuleImplementation1 {
    type Query = QueryNodesInScope<Phenopacket, OntologyClass, First<OntologyClass>>;

    fn check_erased(&'_ self, board: Self::Query) -> bool {
        todo!()
    }
}

struct __RuleImplementation3;

impl TheRuleTrait for __RuleImplementation3 {
    type Query = QueryNodesInScope<Phenopacket, OntologyClass, First<OntologyClass>>;

    fn check_erased(&self, board: Self::Query) -> bool {
        todo!()
    }
}
