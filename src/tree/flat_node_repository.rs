#![allow(dead_code)]
use crate::materializer::NodeMaterializer;
use crate::tree::abstract_pheno_tree::AbstractTreeTraversal;
use crate::tree::error::NodeRepositoryError;
use crate::tree::node::MaterializedNode;
use crate::tree::pointer::Pointer;
use crate::tree::scopes::{ScopeLayer, ScopeMappings};
use crate::tree::traits::{LocatableNode, NodeRepository, NodeRepositoryBuilder};
use serde_json::Value;
use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;

#[derive(Debug)]
struct NodeEntry {
    type_id: TypeId,
    scope: ScopeLayer,
    is_scope_boundary: bool,
    inner: Box<dyn Any>,
}

pub struct FlatNodeRepository {
    node_store: BTreeMap<String, NodeEntry>,
    span_store: BTreeMap<String, Range<usize>>,
    scope_mappings: ScopeMappings,
}

impl FlatNodeRepository {
    pub(crate) fn new() -> Self {
        Self {
            node_store: BTreeMap::new(),
            span_store: BTreeMap::new(),
            scope_mappings: ScopeMappings::new(),
        }
    }

    fn get_subtree_spans(&self, root_path: &str) -> HashMap<Pointer, Range<usize>> {
        self.span_store
            .range::<String, _>(root_path.to_string()..)
            .take_while(|(k, _)| k.starts_with(root_path))
            .map(|(p, r)| (Pointer::from(p.as_str()), r.clone()))
            .collect()
    }

    fn cast_entry<NodeType>(
        &self,
        path: &str,
        entry: &NodeEntry,
    ) -> Result<MaterializedNode<NodeType>, NodeRepositoryError>
    where
        NodeType: Clone + 'static,
    {
        let content_ref = entry.inner.downcast_ref::<NodeType>().ok_or_else(|| {
            NodeRepositoryError::CantReinstantiateNode(
                path.to_string(),
                std::any::type_name::<NodeType>().to_string(),
            )
        })?;

        let content = content_ref.clone();
        let spans = self.get_subtree_spans(path);

        Ok(MaterializedNode::new(content, spans, Pointer::from(path)))
    }
}

impl NodeRepository for FlatNodeRepository {
    fn insert<NodeType: 'static>(
        &mut self,
        node: MaterializedNode<NodeType>,
    ) -> Result<(), NodeRepositoryError> {
        let type_id = TypeId::of::<NodeType>();

        let scope = self.scope_mappings.derive_scope(node.pointer(), &type_id);
        let is_scope_boundary = self.scope_mappings.is_scope_boundary(&type_id);

        for (ptr, span) in node.spans() {
            self.span_store
                .entry(ptr.position().to_string())
                .or_insert_with(|| span.clone());
        }

        let ptr = node.pointer().clone();

        let entry = NodeEntry {
            type_id,
            scope,
            is_scope_boundary,
            inner: Box::new(node.inner),
        };

        self.node_store.insert(ptr.to_string(), entry);

        Ok(())
    }

    fn get_all<NodeType>(&self) -> Result<Vec<MaterializedNode<NodeType>>, NodeRepositoryError>
    where
        NodeType: Clone + 'static,
    {
        let target_type = TypeId::of::<NodeType>();

        let nodes = self
            .node_store
            .iter()
            .filter(|(_, entry)| entry.type_id == target_type)
            .map(|(path, entry)| self.cast_entry::<NodeType>(path.as_str(), entry))
            .collect::<Result<Vec<MaterializedNode<NodeType>>, NodeRepositoryError>>()?;

        Ok(nodes)
    }

    fn get_nodes_in_scope<NodeType>(
        &self,
        scope: ScopeLayer,
    ) -> Result<Vec<MaterializedNode<NodeType>>, NodeRepositoryError>
    where
        NodeType: Clone + 'static,
    {
        let target_type = TypeId::of::<NodeType>();

        let nodes = self
            .node_store
            .iter()
            .filter(|(_, entry)| entry.type_id == target_type && entry.scope == scope)
            .map(|(path, entry)| self.cast_entry::<NodeType>(path, entry))
            .collect::<Result<Vec<MaterializedNode<NodeType>>, NodeRepositoryError>>()?;

        Ok(nodes)
    }

    fn get_nodes_for_scope_per_top_level_element<NodeType>(
        &self,
        scope: ScopeLayer,
    ) -> Result<Vec<Vec<MaterializedNode<NodeType>>>, NodeRepositoryError>
    where
        NodeType: Clone + 'static,
    {
        let target_type = TypeId::of::<NodeType>();
        let top_levels: Vec<&String> = self
            .node_store
            .iter()
            .filter(|(_, entry)| entry.is_scope_boundary && entry.scope == scope)
            .map(|(path, _)| path)
            .collect();

        let mut output = Vec::new();

        for tl_path in top_levels {
            let children = self
                .node_store
                .range::<String, _>(tl_path.to_string()..)
                .take_while(|(k, _)| k.starts_with(tl_path))
                .filter(|(_, entry)| entry.type_id == target_type && entry.scope == scope)
                .map(|(path, entry)| self.cast_entry::<NodeType>(path, entry))
                .collect::<Result<Vec<MaterializedNode<NodeType>>, NodeRepositoryError>>()?;

            output.push(children);
        }

        Ok(output)
    }
}

pub(crate) struct FlatNodeRepositoryBuilder;

impl NodeRepositoryBuilder<FlatNodeRepository> for FlatNodeRepositoryBuilder {
    fn build(tree: Value, spans: HashMap<Pointer, Range<usize>>) -> FlatNodeRepository {
        let mut repo = FlatNodeRepository::new();
        let mut materialized = NodeMaterializer;
        for node in AbstractTreeTraversal::new(tree, spans).traverse() {
            materialized.materialize_nodes(&node, &mut repo);
        }
        repo
    }
}

#[cfg(test)]
mod test_builder {
    use super::*;
    use phenopackets::schema::v2::Phenopacket;
    use phenopackets::schema::v2::core::{MetaData, Resource};

    #[test]
    fn test_builder_single_phenopacket() {
        let test_pp = Phenopacket {
            id: "some_id".to_string(),
            meta_data: Some(MetaData {
                created: Some(Default::default()),
                created_by: "Daniel The Man".to_string(),
                submitted_by: "Peter Hobbitson".to_string(),
                resources: vec![Resource {
                    id: "HP".to_string(),
                    name: "HPO".to_string(),
                    url: "www.hpo.com".to_string(),
                    version: "2.0".to_string(),
                    namespace_prefix: "hp".to_string(),
                    iri_prefix: "prefix".to_string(),
                }],
                phenopacket_schema_version: "2".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let values = serde_json::to_value(&test_pp).unwrap();
        let repo = FlatNodeRepositoryBuilder::build(values, HashMap::new());

        assert_eq!(repo.node_store.len(), 2);

        let boundries: Vec<_> = repo
            .node_store
            .values()
            .filter(|node| node.is_scope_boundary)
            .collect();

        assert_eq!(boundries.len(), 1);

        let pp_node = boundries.first().unwrap();
        assert_eq!(pp_node.type_id, TypeId::of::<Phenopacket>());
        assert_eq!(pp_node.scope, ScopeLayer::Individual);
    }
}

#[cfg(test)]
mod tests_repository {
    use super::*;
    use crate::tree::pointer::Pointer;
    use crate::tree::traits::NodeRepositoryBuilder;
    use phenopackets::schema::v2::core::{MetaData, OntologyClass, Resource};
    use phenopackets::schema::v2::{Cohort, Phenopacket};
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    fn generate_test_cohort() -> Cohort {
        let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("assets");

        let json_phenopacket_path = assets_dir.join("phenopacket.json");
        let phenostr = fs::read_to_string(json_phenopacket_path).unwrap();

        let pp: Phenopacket = serde_json::from_str(&phenostr).unwrap();

        Cohort {
            id: "Some".to_string(),
            description: "".to_string(),
            members: vec![pp.clone(), pp.clone()],
            files: vec![],
            meta_data: Some(MetaData {
                created: None,
                created_by: "Patrick".to_string(),
                submitted_by: "Patrick".to_string(),
                resources: vec![Resource {
                    id: "1".to_string(),
                    name: "HP".to_string(),
                    url: "www.example.com".to_string(),
                    version: "2020-10-10".to_string(),
                    namespace_prefix: "hp".to_string(),
                    iri_prefix: "hp".to_string(),
                }],
                updates: vec![],
                phenopacket_schema_version: "2".to_string(),
                external_references: vec![],
            }),
        }
    }

    struct NodeRepoUnitTester;

    impl NodeRepoUnitTester {
        pub fn test<R, B>(cohort: Cohort)
        where
            R: NodeRepository,
            B: NodeRepositoryBuilder<R>,
        {
            let node_repo: R = Self::build_test_repo::<R, B>(&cohort);
            Self::test_get_nodes_for_scope_per_top_level_element(&node_repo, &cohort);
            Self::test_get_all_nodes(&node_repo, &cohort);
            Self::test_get_nodes_in_scope(&node_repo, &cohort);
        }

        fn build_test_repo<R, B>(cohort: &Cohort) -> R
        where
            R: NodeRepository,
            B: NodeRepositoryBuilder<R>,
        {
            let value = serde_json::to_value(cohort).unwrap();
            B::build(value, HashMap::new())
        }

        fn test_get_nodes_in_scope<R>(repo: &R, cohort: &Cohort)
        where
            R: NodeRepository,
        {
            let cohort_level_resources = repo
                .get_nodes_in_scope::<Resource>(ScopeLayer::Aggregated)
                .unwrap();

            for node_resource in cohort_level_resources.iter() {
                assert!(
                    cohort
                        .meta_data
                        .clone()
                        .unwrap()
                        .resources
                        .contains(&node_resource.inner)
                );
            }
            assert_eq!(
                cohort.meta_data.clone().unwrap().resources.len(),
                cohort_level_resources.len()
            );

            let pp_level_resources = repo
                .get_nodes_in_scope::<Resource>(ScopeLayer::Individual)
                .unwrap();

            let all_pp_resources: Vec<Resource> = cohort
                .members
                .clone()
                .iter()
                .flat_map(|pp| pp.meta_data.clone().unwrap().resources)
                .collect();

            for node_resource in pp_level_resources.iter() {
                assert!(all_pp_resources.contains(&node_resource.inner));
            }
            assert_eq!(pp_level_resources.len(), all_pp_resources.len());
        }

        fn test_get_nodes_for_scope_per_top_level_element<R>(repo: &R, cohort: &Cohort)
        where
            R: NodeRepository,
        {
            let retrieved = repo
                .get_nodes_for_scope_per_top_level_element::<Resource>(ScopeLayer::Individual)
                .unwrap();

            for (member, nodes) in cohort.members.iter().zip(&retrieved) {
                let resources = &member.meta_data.as_ref().unwrap().resources;
                for (resource, node) in resources.iter().zip(nodes) {
                    assert_eq!(&node.inner, resource);
                }
            }
        }

        fn test_get_all_nodes<R>(repo: &R, cohort: &Cohort)
        where
            R: NodeRepository,
        {
            let retrieved = repo.get_all::<Resource>().unwrap();

            let mut n_resources = cohort.meta_data.clone().unwrap().resources.len();
            for pp in cohort.members.clone() {
                n_resources += pp.meta_data.unwrap().resources.len()
            }

            assert_eq!(
                retrieved.len(),
                n_resources,
                "Got {} resources from repo, but expected {}",
                retrieved.len(),
                n_resources
            );
        }
    }

    #[test]
    fn test_node_repository() {
        let cohort = generate_test_cohort();
        NodeRepoUnitTester::test::<FlatNodeRepository, FlatNodeRepositoryBuilder>(cohort);
    }

    #[test]
    fn test_insert() {
        let mut repo = FlatNodeRepository::new();

        let node_pointer = Pointer::from("phenotypicFeatures/0/type");
        let mut spans = BTreeMap::new();
        spans.insert(node_pointer.to_string().clone(), 0usize..50usize);

        let node = MaterializedNode::new(
            OntologyClass {
                id: "HP:0000001".to_string(),
                label: "All".to_string(),
            },
            spans
                .iter()
                .map(|(key, val)| (Pointer::from(key.as_str()), val.clone()))
                .collect(),
            node_pointer.clone(),
        );
        repo.insert(node).unwrap();

        let node_entry = repo.node_store.get(&node_pointer.to_string()).unwrap();

        assert_eq!(repo.span_store, spans);
        assert_eq!(node_entry.type_id, TypeId::of::<OntologyClass>());
        assert_eq!(node_entry.scope, ScopeLayer::Individual);
        assert!(!node_entry.is_scope_boundary);
    }
}
