use crate::tree::node::MaterializedNode;

use crate::tree::scopes::ScopeDefinition;
use crate::tree::traits::NodeRepository;

use crate::tree::querying::traits::{QueryPresentation, QueryStrategy};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct QueryAllNodes<NodeType, Presentation>(PhantomData<(Presentation, NodeType)>);

impl<NodeType: Clone + 'static, Presentation: QueryPresentation<Vec<MaterializedNode<NodeType>>>>
    QueryStrategy for QueryAllNodes<NodeType, Presentation>
{
    type Output = Presentation;
    fn query(node_repo: &impl NodeRepository) -> Self::Output {
        Presentation::present(node_repo.get_all::<NodeType>().unwrap_or_default())
    }
}
#[derive(Debug)]
pub struct QueryNodesInScope<Scope: ScopeDefinition, NodeType, Presentation>(
    PhantomData<(Scope, Presentation, NodeType)>,
);

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
pub struct QueryGroupedNodes<Scope: ScopeDefinition, NodeType, Presentation>(
    PhantomData<(Scope, Presentation, NodeType)>,
);

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

pub mod convenience {
    use crate::tree::querying::presentation::{First, Flattened, Grouped};
    use crate::tree::querying::queries::{QueryAllNodes, QueryGroupedNodes, QueryNodesInScope};
    use phenopackets::schema::v2::Phenopacket;

    // More to be added.
    pub type All<NodeType> = QueryAllNodes<NodeType, Flattened<NodeType>>;
    pub type GroupedIndividuals<NodeType> =
        QueryGroupedNodes<Phenopacket, NodeType, Grouped<NodeType>>;
    pub type SingleInScope<Scope, NodeType> = QueryNodesInScope<Scope, NodeType, First<NodeType>>;
}

mod temp_test {
    #![allow(dead_code)]
    #![allow(unused)]
    // Testing and see how it would work from here:

    use crate::tree::querying::presentation::{First, Flattened};
    use crate::tree::querying::queries::convenience::{All, GroupedIndividuals, SingleInScope};
    use crate::tree::querying::queries::{QueryAllNodes, QueryNodesInScope};
    use crate::tree::querying::traits::QueryStrategy;
    use phenopackets::schema::v2::Phenopacket;
    use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};

    trait TheRuleTrait {
        type Query: QueryStrategy;

        fn check_erased(&'_ self, board: <Self::Query as QueryStrategy>::Output) -> bool;
    }

    struct __RuleImplementation1;

    impl TheRuleTrait for __RuleImplementation1 {
        type Query = (
            QueryNodesInScope<Phenopacket, OntologyClass, First<OntologyClass>>,
            QueryAllNodes<OntologyClass, Flattened<OntologyClass>>,
        );

        fn check_erased(&'_ self, board: <Self::Query as QueryStrategy>::Output) -> bool {
            todo!()
        }
    }

    struct __RuleImplementation2;

    impl TheRuleTrait for __RuleImplementation2 {
        type Query = (
            All<OntologyClass>,
            SingleInScope<Phenopacket, OntologyClass>,
            GroupedIndividuals<PhenotypicFeature>,
        );

        fn check_erased(&self, board: <Self::Query as QueryStrategy>::Output) -> bool {
            let a = board;
            todo!()
        }
    }

    struct __RuleImplementation3;

    impl TheRuleTrait for __RuleImplementation3 {
        type Query = GroupedIndividuals<OntologyClass>;

        fn check_erased(&self, board: <Self::Query as QueryStrategy>::Output) -> bool {
            todo!()
        }
    }
}
