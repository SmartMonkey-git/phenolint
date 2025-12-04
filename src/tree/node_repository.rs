use crate::rules::traits::LintData;
use crate::tree::node::MaterializedNode;
use crate::tree::pointer::Pointer;
use crate::tree::traits::{Node, Scoped};
use serde::Serialize;
use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Default)]
pub struct NodeRepository {
    // Stores the individual nodes of a phenopacket/cohort.
    // First key (u8) indicates the scope -> Cohort or Phenopacket
    // Second key (String) is a unique ID identifyin either a phenopacket or a cohort
    // Third key is the type id of the node.
    // Problem here is, that everything is stored several times. If the cohort has 100 Phenopackets, then the cohort node will store them, as well as the phenopackets themself in scope 2
    // reason for this is that some rules want to look at the nodes of an individual phenopacket, while other want to have all nodes of a type of the whole cohort.

    // scope_id -> members of that scope -> individual nodes
    board: HashMap<u8, BTreeMap<String, HashMap<TypeId, Box<dyn Any>>>>,
}

impl NodeRepository {
    pub fn new() -> NodeRepository {
        NodeRepository {
            board: HashMap::new(),
        }
    }

    fn get_raw<T: 'static>(&self, scope_id: &u8) -> Vec<&[MaterializedNode<T>]> {
        let scope_entries = self.board.get(scope_id).unwrap();

        scope_entries
            .iter()
            .map(|(_, case_entry)| match case_entry.get(&TypeId::of::<T>()) {
                Some(nodes) => nodes
                    .downcast_ref::<Vec<MaterializedNode<T>>>()
                    .unwrap()
                    .as_slice(),
                None => &[],
            })
            .collect()
    }

    pub fn insert<T: 'static>(
        &mut self,
        scope: u8,
        top_level_node_id: &str,
        node: MaterializedNode<T>,
    ) {
        let scope_entries = self.board.entry(scope).or_insert_with(|| BTreeMap::new());
        let case_entry = scope_entries
            .entry(top_level_node_id.to_string())
            .or_insert_with(HashMap::new);

        case_entry
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(Vec::<MaterializedNode<T>>::new()))
            .downcast_mut::<Vec<MaterializedNode<T>>>()
            .unwrap()
            .push(node);
    }
    pub fn node_by_pointer<T: 'static + Clone + Serialize>(
        &self,
        ptr: &Pointer,
    ) -> Option<&MaterializedNode<T>> {
        let target_type_id = TypeId::of::<Vec<MaterializedNode<T>>>();

        for scope_map in self.board.values() {
            for type_map in scope_map.values() {
                if let Some(any_storage) = type_map.get(&target_type_id) {
                    if let Some(nodes) = any_storage.downcast_ref::<Vec<MaterializedNode<T>>>() {
                        for node in nodes {
                            if node.pointer() == ptr {
                                return Some(node);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

/*
pub struct Single<'a, T: 'static>(pub Option<&'a MaterializedNode<T>>);

impl<'a, T> LintData<'a> for Single<'a, T> {
    fn fetch(board: &'a NodeRepository) -> Self {
        Single(board.get_raw::<T>().first())
    }
}*/

pub struct List<'a, Data: 'static> {
    pub inner: &'a [MaterializedNode<Data>],
}

impl<'a, Data> List<'a, Data> {
    pub fn new(data: &'a [MaterializedNode<Data>]) -> Self {
        Self { inner: data }
    }
}

impl<'a, Data> Deref for List<'a, Data> {
    type Target = &'a [MaterializedNode<Data>];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait FromLintData {
    fn from_lint_data<'a>(data: impl LintData<'a>) -> Self;
}

impl<Data> FromLintData for List<'_, Data> {
    fn from_lint_data<'a>(data: impl LintData<'a>) -> Self {
        todo!()
    }
}

/*

impl<'a, Data> LintData<'a> for List<'a, Data> {
    fn fetch(board: &'a NodeRepository) -> Self {
        // TODO: Get data for scope
        List::new(board.get_raw(scope_id))
    }
}*/

impl<'a, A, B> LintData<'a> for (A, B)
where
    A: LintData<'a>,
    B: LintData<'a>,
{
    fn fetch(board: &'a NodeRepository) -> Self {
        (A::fetch(board), B::fetch(board))
    }
}

impl<'a, A, B, C> LintData<'a> for (A, B, C)
where
    A: LintData<'a>,
    B: LintData<'a>,
    C: LintData<'a>,
{
    fn fetch(board: &'a NodeRepository) -> Self {
        (A::fetch(board), B::fetch(board), C::fetch(board))
    }
}
