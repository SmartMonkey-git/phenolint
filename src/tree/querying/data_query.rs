use crate::tree::node::MaterializedNode;
use crate::tree::querying::presentation::{First, Flattened, Grouped, QueryPresentation};
use crate::tree::scopes::ScopeDefinition;
use crate::tree::traits::NodeRepository;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::OntologyClass;
use std::marker::PhantomData;

trait QueryStrategy {
    type Output;
    fn query(node_repo: &impl NodeRepository) -> Self::Output;
}

macro_rules! impl_query_strategy_for_tuples {
    ($($name:ident),*) => {
        impl<$($name: QueryStrategy),*> QueryStrategy for ($($name,)*) {
            type Output = ($($name::Output,)*);

            fn query(node_repo: &impl NodeRepository) -> Self::Output {
                (
                    $($name::query(node_repo),)*
                )
            }
        }
    };
}

impl_query_strategy_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

#[derive(Debug)]
struct QueryAllNodes<NodeType, Presentation> {
    node_type: PhantomData<NodeType>,
    presentation: PhantomData<Presentation>,
}

impl<NodeType: Clone + 'static, Presentation: QueryPresentation<Vec<MaterializedNode<NodeType>>>>
    QueryStrategy for QueryAllNodes<NodeType, Presentation>
{
    type Output = Presentation;
    fn query(node_repo: &impl NodeRepository) -> Self::Output {
        Presentation::present(node_repo.get_all::<NodeType>().unwrap_or_default())
    }
}
#[derive(Debug)]
struct QueryNodesInScope<Scope: ScopeDefinition, NodeType, Presentation> {
    result: PhantomData<Presentation>,
    scope: PhantomData<Scope>,
    node_type: PhantomData<NodeType>,
}

impl<
    Scope: ScopeDefinition,
    NodeType: Clone + 'static,
    Presentation: QueryPresentation<Vec<MaterializedNode<NodeType>>>,
> QueryStrategy for QueryNodesInScope<Scope, NodeType, Presentation>
{
    type Output = Presentation;
    fn query(node_repo: &impl NodeRepository) -> Self::Output {
        let query_result = node_repo
            .get_nodes_in_scope::<NodeType>(Scope::layer())
            .unwrap_or_default();

        Presentation::present(query_result)
    }
}

#[derive(Debug)]
struct QueryGroupedNodes<Scope: ScopeDefinition, NodeType, Presentation> {
    pub result: Presentation,
    _scope: PhantomData<Scope>,
    _node: PhantomData<NodeType>,
}

impl<
    Scope: ScopeDefinition,
    NodeType: Clone + 'static,
    Presentation: QueryPresentation<Vec<Vec<MaterializedNode<NodeType>>>>,
> QueryStrategy for QueryGroupedNodes<Scope, NodeType, Presentation>
{
    type Output = Presentation;
    fn query(node_repo: &impl NodeRepository) -> Self::Output {
        let query_result = node_repo
            .get_nodes_for_scope_per_top_level_element::<NodeType>(Scope::layer())
            .unwrap_or_default();

        Presentation::present(query_result)
    }
}

// Testing and see how it would work from here:

trait TheRuleTrait {
    type Query: QueryStrategy;

    fn check_erased(&'_ self, board: <Self::Query as QueryStrategy>::Output) -> bool;
}

struct __RuleImplementation1;

impl TheRuleTrait for __RuleImplementation1 {
    type Query = QueryNodesInScope<Phenopacket, OntologyClass, First<OntologyClass>>;

    fn check_erased(&'_ self, board: <Self::Query as QueryStrategy>::Output) -> bool {
        todo!()
    }
}

struct __RuleImplementation2;

// More to be added.
type QueryAll<NodeType> = QueryAllNodes<NodeType, Flattened<NodeType>>;

impl TheRuleTrait for __RuleImplementation2 {
    type Query = QueryAll<OntologyClass>;

    fn check_erased(&self, board: <Self::Query as QueryStrategy>::Output) -> bool {
        let a = board;
        todo!()
    }
}

struct __RuleImplementation3;

impl TheRuleTrait for __RuleImplementation3 {
    type Query = QueryGroupedNodes<Phenopacket, OntologyClass, Grouped<OntologyClass>>;

    fn check_erased(&self, board: <Self::Query as QueryStrategy>::Output) -> bool {
        todo!()
    }
}
