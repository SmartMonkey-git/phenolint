use crate::rules::traits::LintData;
use crate::tree::node::MaterializedNode;
use crate::tree::pointer::Pointer;
use crate::tree::traits::Node;
use serde::Serialize;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Default)]
pub struct NodeRepository {
    board: HashMap<TypeId, Box<dyn Any>>,
}

impl NodeRepository {
    pub fn new() -> NodeRepository {
        NodeRepository {
            board: HashMap::new(),
        }
    }

    fn get_raw<T: 'static>(&self) -> &[MaterializedNode<T>] {
        self.board
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref::<Vec<MaterializedNode<T>>>())
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn insert<T: 'static>(&mut self, node: MaterializedNode<T>) {
        self.board
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
        for nodes in self.board.values() {
            let casted_node = nodes
                .downcast_ref::<Vec<MaterializedNode<T>>>()
                .expect("Should be downcastable");

            for node in casted_node.iter() {
                if node.pointer() == ptr {
                    return Some(node);
                }
            }
        }
        None
    }
}

pub struct Single<'a, T: 'static>(pub Option<&'a MaterializedNode<T>>);

impl<'a, T> LintData<'a> for Single<'a, T> {
    fn fetch(board: &'a NodeRepository) -> Self {
        Single(board.get_raw::<T>().first())
    }
}

pub struct List<'a, T: 'static>(pub &'a [MaterializedNode<T>]);

impl<'a, T> Deref for List<'a, T> {
    type Target = &'a [MaterializedNode<T>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> LintData<'a> for List<'a, T> {
    fn fetch(board: &'a NodeRepository) -> Self {
        List(board.get_raw())
    }
}

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
