#![allow(dead_code)]
use crate::tree::error::NodeRepositoryError;
use crate::tree::node::MaterializedNode;
use crate::tree::pointer::Pointer;
use crate::tree::traits::{LocatableNode, NodeRepository};
use phenopackets::schema::v2::{Cohort, Family, Phenopacket};
use std::any::{Any, TypeId};
use std::cell::Cell;
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;

pub(crate) struct ScopeMappings {
    scope_by_type_id: HashMap<TypeId, u8>,
    max_scope: Cell<u8>,
}

impl ScopeMappings {
    pub(crate) fn new() -> Self {
        let mut type_id_by_scope: HashMap<TypeId, u8> = HashMap::new();
        type_id_by_scope.insert(TypeId::of::<Phenopacket>(), 0u8);
        type_id_by_scope.insert(TypeId::of::<Cohort>(), 1u8);
        type_id_by_scope.insert(TypeId::of::<Family>(), 1u8);

        Self {
            max_scope: Cell::from(
                *type_id_by_scope
                    .values()
                    .min()
                    .expect("Value was just assigned"),
            ),
            scope_by_type_id: type_id_by_scope,
        }
    }

    pub fn get_scope(&self, type_id: &TypeId) -> Option<u8> {
        self.scope_by_type_id.get(type_id).copied()
    }

    pub fn is_scope_boundary(&self, type_id: &TypeId) -> bool {
        self.scope_by_type_id.contains_key(type_id)
    }

    pub fn derive_scope(&self, path: &str, type_id: &TypeId) -> u8 {
        if let Some(scope) = self.get_scope(type_id) {
            let current_max = self.max_scope.get();
            self.max_scope.set(current_max.max(scope));
        }

        let phenopacket_type_id = TypeId::of::<Phenopacket>();

        if path.contains("members")
            || path.contains("relatives")
            || path.contains("proband")
            // This is needed to know, when we only look at a single phenopacket.
            // Since, we are iterating the phenopacket tree from top to bottom, we will always find top level structures
            // that are above the phenopacket, if not we can assume, that we are only looking at a single one.
            || self.max_scope.get() == *self.scope_by_type_id.get(&phenopacket_type_id).unwrap()
            || type_id == &phenopacket_type_id
        {
            self.get_scope(&TypeId::of::<Phenopacket>())
                .expect("Should always exist")
        } else {
            self.get_scope(&TypeId::of::<Cohort>())
                .expect("Should always exist")
        }
    }
}

struct NodeEntry {
    type_id: TypeId,
    scope: u8,
    is_scope_boundary: bool,
    inner: Box<dyn Any>,
}

pub(crate) struct BTreeNodeRepository {
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
            .map(|(p, r)| (Pointer::from(p.as_str()), r.clone()))
            .collect()
    }

    fn cast_entry<T>(
        &self,
        path: &str,
        entry: &NodeEntry,
    ) -> Result<MaterializedNode<T>, NodeRepositoryError>
    where
        T: Clone + 'static,
    {
        let content_ref = entry.inner.downcast_ref::<T>().ok_or_else(|| {
            NodeRepositoryError::CantReinstantiateNode(
                path.to_string(),
                std::any::type_name::<T>().to_string(),
            )
        })?;

        let content = content_ref.clone();
        let spans = self.get_subtree_spans(path);

        Ok(MaterializedNode::new(content, spans, Pointer::from(path)))
    }
}

impl NodeRepository for BTreeNodeRepository {
    fn insert<T: 'static>(&mut self, node: MaterializedNode<T>) -> Result<(), NodeRepositoryError> {
        let type_id = TypeId::of::<T>();
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

    fn get_all<T>(&self) -> Result<Vec<MaterializedNode<T>>, NodeRepositoryError>
    where
        T: Clone + 'static,
    {
        let target_type = TypeId::of::<T>();

        let nodes = self
            .node_store
            .iter()
            .filter(|(_, entry)| entry.type_id == target_type)
            .map(|(path, entry)| self.cast_entry::<T>(path.as_str(), entry))
            .collect::<Result<Vec<MaterializedNode<T>>, NodeRepositoryError>>()?;

        Ok(nodes)
    }

    fn get_nodes_in_scope<T>(
        &self,
        scope: u8,
    ) -> Result<Vec<MaterializedNode<T>>, NodeRepositoryError>
    where
        T: Clone + 'static,
    {
        let target_type = TypeId::of::<T>();

        let nodes = self
            .node_store
            .iter()
            .filter(|(_, entry)| entry.type_id == target_type && entry.scope == scope)
            .map(|(path, entry)| self.cast_entry::<T>(path, entry))
            .collect::<Result<Vec<MaterializedNode<T>>, NodeRepositoryError>>()?;

        Ok(nodes)
    }

    fn get_nodes_for_scope_per_top_level_element<T>(
        &self,
        scope: u8,
    ) -> Result<Vec<Vec<MaterializedNode<T>>>, NodeRepositoryError>
    where
        T: Clone + 'static,
    {
        let target_type = TypeId::of::<T>();

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
                .map(|(path, entry)| self.cast_entry::<T>(path, entry))
                .collect::<Result<Vec<MaterializedNode<T>>, NodeRepositoryError>>()?;

            if !children.is_empty() {
                output.push(children);
            }
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
            Pointer::from("phenotypicFeatures/0/type").clone(),
        );
        repo.insert(node).unwrap();
    }

    #[test]
    fn test_get_nodes_for_scope_per_top_level_element() {
        let repo = cohort_board();
        let retrieved = repo
            .get_nodes_for_scope_per_top_level_element::<OntologyClass>(0u8)
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
