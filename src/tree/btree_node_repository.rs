#![allow(dead_code)]
use crate::tree::error::NodeRepositoryError;
use crate::tree::node::MaterializedNode;
use crate::tree::pointer::Pointer;
use crate::tree::scopes::{ScopeLayer, ScopeMappings};
use crate::tree::traits::{LocatableNode, NodeRepository};
use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;

struct NodeEntry {
    type_id: TypeId,
    scope: ScopeLayer,
    is_scope_boundary: bool,
    inner: Box<dyn Any>,
}

pub struct BTreeNodeRepository {
    node_store: BTreeMap<String, NodeEntry>,
    span_store: BTreeMap<String, Range<usize>>,
    scope_mappings: ScopeMappings,
}

impl BTreeNodeRepository {
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
            .map(|(p, r)| (Pointer::new(p.as_str()), r.clone()))
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

        Ok(MaterializedNode::new(content, spans, Pointer::new(path)))
    }
}

impl NodeRepository for BTreeNodeRepository {
    fn insert<NodeType: 'static>(
        &mut self,
        node: MaterializedNode<NodeType>,
    ) -> Result<(), NodeRepositoryError> {
        let type_id = TypeId::of::<NodeType>();
        let node_path = node.pointer().position().to_string();

        let scope = self
            .scope_mappings
            .derive_scope(node_path.as_str(), &type_id);
        let is_scope_boundary = self.scope_mappings.is_scope_boundary(&type_id);

        for (ptr, span) in node.spans() {
            self.span_store
                .entry(ptr.position().to_string())
                .or_insert_with(|| span.clone());
        }

        let entry = NodeEntry {
            type_id,
            scope,
            is_scope_boundary,
            inner: Box::new(node.inner),
        };

        self.node_store.insert(node_path.to_string(), entry);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::pointer::Pointer;
    use phenopackets::schema::v2::core::{MetaData, OntologyClass, Resource};
    use phenopackets::schema::v2::{Cohort, Phenopacket};
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    fn test_cohort() -> Cohort {
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
                    iri_prefix: "".to_string(),
                }],
                updates: vec![],
                phenopacket_schema_version: "2".to_string(),
                external_references: vec![],
            }),
        }
    }
    fn cohort_board() -> BTreeNodeRepository {
        /*let cohort = test_cohort();
        let value = serde_json::to_value(&cohort).unwrap();

         let tree = AbstractTreeTraversal::new(value, HashMap::new());
        let repo = BTreeNodeRepository::new();

        let mat = NodeMaterializer;
        // TODO: Change interface of materialize_nodes to take an impl NodeRepository trait
        for node in tree.traverse() {
            mat.materialize_nodes(&node, &mut repo);
        }
        repo
         */
        BTreeNodeRepository::new()
    }

    #[test]
    fn test_insert() {
        let mut repo = BTreeNodeRepository::new();

        let node = MaterializedNode::new(
            OntologyClass {
                id: "HP:0000001".to_string(),
                label: "All".to_string(),
            },
            HashMap::new(),
            Pointer::at_phenotypes().down("0/type").clone(),
        );
        repo.insert(node).unwrap();
    }

    #[test]
    fn test_get_nodes_for_scope_per_top_level_element() {
        let repo = cohort_board();
        let retrieved = repo
            .get_nodes_for_scope_per_top_level_element::<OntologyClass>(ScopeLayer::Individual)
            .unwrap();

        assert_eq!(retrieved.len(), 2);
    }

    #[test]
    fn test_get_all_nodes() {
        let repo = cohort_board();
        let test_cohort = test_cohort();
        let retrieved = repo.get_all::<Resource>().unwrap();

        let mut n_resources = test_cohort.meta_data.unwrap().resources.len();

        for pp in test_cohort.members {
            n_resources += pp.meta_data.unwrap().resources.len()
        }

        assert_eq!(retrieved.len(), n_resources);
    }
}
