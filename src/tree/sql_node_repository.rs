use crate::tree::node::MaterializedNode;
use crate::tree::pointer::Pointer;
use crate::tree::traits::{Node, NodeRepository};
use phenopackets::schema::v2::{Cohort, Family, Phenopacket};
use prost::Message;
use rusqlite::Connection as SQLiteConnection;
use serde::de::DeserializeOwned;
use std::any::TypeId;
use std::cell::Cell;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

fn type_id_to_u64(type_id: &TypeId) -> u64 {
    let mut hasher = DefaultHasher::new();
    type_id.hash(&mut hasher);
    hasher.finish()
}

pub(crate) struct ScopeMappings {
    scope_by_type_id: HashMap<TypeId, u8>,
    max_scope: Cell<u8>,
}

impl ScopeMappings {
    fn new() -> Self {
        let mut type_id_by_scope: HashMap<TypeId, u8> = HashMap::new();
        type_id_by_scope.insert(TypeId::of::<Phenopacket>(), 0u8);
        type_id_by_scope.insert(TypeId::of::<Cohort>(), 1u8);
        type_id_by_scope.insert(TypeId::of::<Family>(), 1u8);

        Self {
            scope_by_type_id: type_id_by_scope,
            max_scope: Cell::from(0u8),
        }
    }

    pub fn get_scope(&self, type_id: &TypeId) -> Option<u8> {
        self.scope_by_type_id.get(type_id).copied()
    }

    pub fn get_type_id(&self, scope: &u8) -> Option<&TypeId> {
        self.scope_by_type_id
            .iter()
            .find_map(|(type_id, v)| if v == scope { Some(type_id) } else { None })
    }

    pub fn is_scope_boundary(&self, type_id: &TypeId) -> bool {
        self.scope_by_type_id.contains_key(type_id)
    }

    pub fn derive_scope<T: 'static>(&self, node: &MaterializedNode<T>) -> u8 {
        let type_id = TypeId::of::<T>();
        let path_str = node.pointer().position();

        if let Some(scope) = self.get_scope(&type_id) {
            let current_max = self.max_scope.get();
            self.max_scope.set(current_max.max(scope));
        }

        let phenopacket_type_id = TypeId::of::<Phenopacket>();
        let case_scope = self.scope_by_type_id.get(&phenopacket_type_id).unwrap();

        if phenopacket_type_id == type_id {
            return *case_scope;
        }

        if path_str.contains("members")
            || path_str.contains("relatives")
            || path_str.contains("proband")
            // This is needed to know, when we only look at a single phenopacket.
            // Since, we are iterating the phenopacket tree from top to bottom, we will always find top level structures
            // that are above the phenopacket, if not we can assume, that we are only looking at a single one
            || self.max_scope.get() == *case_scope
        {
            self.get_scope(&TypeId::of::<Phenopacket>())
                .expect("Should always exist")
        } else {
            self.get_scope(&TypeId::of::<Cohort>())
                .expect("Should always exist")
        }
    }
}

struct SQLNode {
    /// Unique id of the node
    id: i64,
    /// Type of the inner
    type_id_hash: i64,
    /// The scope the node belongs to in the tree
    level: u8,
    /// Path to the node in the tree
    path: String,
    /// The actual node of the former tree
    inner: Vec<u8>,
}

struct SQLSpan {
    path: String,
    start: i64,
    end: i64,
}

pub(crate) struct SQLNodeRepository {
    board: SQLiteConnection,
    scope_mappings: ScopeMappings,
}

impl SQLNodeRepository {
    const NODE_TABLE_NAME: &'static str = "sql_node";

    fn node_table_query() -> String {
        format!(
            r#"
            CREATE TABLE {} (
            id INTEGER PRIMARY KEY,
            type_id_hash INTEGER NOT NULL,
            scope INTEGER NOT NULL,
            is_scope_boundary BOOLEAN NOT NULL,
            path TEXT NOT NULL,
            inner BLOB NOT NULL
            );"#,
            Self::NODE_TABLE_NAME,
        )
    }

    const SPAN_TABLE_NAME: &'static str = "__sql_node_span";

    fn span_table_query() -> String {
        format!(
            r#"
            CREATE TABLE {} (
            path TEXT NOT NULL,
            start INTEGER NOT NULL,
            end INTEGER NOT NULL
            );
            "#,
            Self::SPAN_TABLE_NAME
        )
    }

    // TODO: Get rid of String returns and unwraps everywhere
    pub(crate) fn new() -> Result<Self, String> {
        if let Ok(board) = SQLiteConnection::open_in_memory() {
            board
                .execute(&Self::node_table_query(), ())
                .expect("Should always create the node table");

            board
                .execute(&Self::span_table_query(), ())
                .expect("Should always create the span table");

            board
                .execute(
                    &format!(
                        "CREATE INDEX idx_{}_main ON {} (type_id_hash, scope, path);",
                        Self::NODE_TABLE_NAME,
                        Self::NODE_TABLE_NAME
                    ),
                    (),
                )
                .expect("Should always create the index");

            Ok(Self {
                board,
                scope_mappings: ScopeMappings::new(),
            })
        } else {
            Err("Could not open database".into())
        }
    }
}

impl NodeRepository for SQLNodeRepository {
    fn insert<T: 'static + Message>(&mut self, node: MaterializedNode<T>) -> Result<(), String> {
        let type_id = TypeId::of::<T>();
        use std::any::type_name;
        let mut bytes = Vec::new();
        T::encode(&node.inner, &mut bytes).unwrap();

        let type_id_hash = type_id_to_u64(&type_id);
        let scope = self.scope_mappings.derive_scope::<T>(&node);
        let is_scope_boundary = self.scope_mappings.is_scope_boundary(&type_id);

        //println!("----Saving----");
        //println!("Type: {}", type_name::<T>());
        //println!("is_scope_boundary: {}", is_scope_boundary);
        //println!("In scope: {}", scope);

        self.board
            .execute(
                &format!(
                    "INSERT INTO {} (type_id_hash, scope, is_scope_boundary, path, inner) VALUES (?1, ?2, ?3, ?4, ?5)",
                    Self::NODE_TABLE_NAME
                ),
                (
                    type_id_hash as i64,
                    &scope,
                    is_scope_boundary,
                    node.pointer().position(),
                    bytes,
                ),
            )
            .expect("Should always insert");

        Ok(())
    }

    fn get_nodes_in_scope<T: DeserializeOwned + 'static>(
        &self,
        scope: u8,
    ) -> Result<Vec<MaterializedNode<T>>, String> {
        todo!()
    }

    // Should get all nodes within a scope, encapsulated in a vec per top level element of that scope
    // The path needs to be used to determine which nodes goes into which vec
    fn get_nodes_for_scope_per_top_level_element<T: 'static + Message + Default>(
        &self,
        scope: u8,
    ) -> Result<Vec<Vec<MaterializedNode<T>>>, String> {
        let type_id = TypeId::of::<T>();
        let type_id_hash = type_id_to_u64(&type_id);

        let query = format!(
            "WITH top_level AS (
            SELECT path FROM {}
            WHERE is_scope_boundary = ?1 AND scope = ?2
             )
            SELECT n.path, n.inner, tl.path as top_level_path
            FROM {} n
            JOIN top_level tl ON n.path LIKE tl.path || '%'
            WHERE n.type_id_hash = ?3 AND n.scope = ?2
            ORDER BY tl.path ASC, n.path ASC",
            Self::NODE_TABLE_NAME,
            Self::NODE_TABLE_NAME
        );

        let mut stmt = self
            .board
            .prepare(&query)
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let nodes = stmt
            .query_map((true, scope, type_id_hash as i64), |row| {
                let path: String = row.get(0)?;
                let bytes: Vec<u8> = row.get(1)?;
                let top_level_path: String = row.get(2)?;
                Ok((bytes, path, top_level_path))
            })
            .map_err(|e| format!("Failed to query: {}", e))?;

        let mut output: HashMap<String, Vec<MaterializedNode<T>>> = HashMap::new();

        for raw_node in nodes {
            let (bytes, path, top_level_path) =
                raw_node.map_err(|e| format!("Failed to read row: {}", e))?;

            let node_content: T = T::decode(bytes.as_slice()).unwrap();
            let node = MaterializedNode::new(node_content, Default::default(), Pointer::new(&path));

            output.entry(top_level_path).or_default().push(node);
        }

        let output = output.into_values().collect::<Vec<_>>();
        Ok(output)
    }

    fn get_all<T: DeserializeOwned + 'static>(&self) -> Vec<MaterializedNode<T>> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::materializer::NodeMaterializer;
    use crate::tree::abstract_pheno_tree::AbstractTreeTraversal;
    use crate::tree::pointer::Pointer;
    use phenopackets::schema::v2::core::OntologyClass;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_insert() {
        let mut repo = SQLNodeRepository::new().expect("Failed to create board");

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
    fn test_insert_2() {
        let mut repo = SQLNodeRepository::new().expect("Failed to create board");
        let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("assets");
        let json_phenopacket_path = assets_dir.join("phenopacket.json");
        let phenostr = fs::read_to_string(json_phenopacket_path).unwrap();

        let pp: Phenopacket = serde_json::from_str(&phenostr).unwrap();

        let cohort = Cohort {
            id: "Some".to_string(),
            description: "".to_string(),
            members: vec![pp.clone(), pp.clone()],
            files: vec![],
            meta_data: None,
        };

        let value = serde_json::to_value(&cohort).unwrap();

        let tree = AbstractTreeTraversal::new(value, HashMap::new());
        let mat = NodeMaterializer;
        for node in tree.traverse() {
            mat.materialize_nodes(&node, &mut repo);
        }

        let retrieved = repo
            .get_nodes_for_scope_per_top_level_element::<OntologyClass>(0u8)
            .unwrap();

        assert_eq!(retrieved.len(), 2);
        dbg!(&retrieved);
    }
}
