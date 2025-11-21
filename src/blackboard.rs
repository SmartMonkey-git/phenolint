use crate::rules::traits::LintData;
use crate::tree::node::MaterializedNode;
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct BlackBoard {
    board: HashMap<TypeId, Box<dyn Any>>,
}

impl BlackBoard {
    pub fn new() -> BlackBoard {
        BlackBoard {
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
}

pub struct List<'a, T: 'static>(pub Vec<&'a T>);

impl<'a, T> LintData<'a> for List<'a, T> {
    fn fetch(board: &'a BlackBoard) -> Self {
        let nodes = board.get_raw::<T>();

        let a: Vec<&T> = nodes.iter().map(|node| &node.materialized_node).collect();

        List(a)
    }
}

impl<'a, A, B> LintData<'a> for (A, B)
where
    A: LintData<'a>,
    B: LintData<'a>,
{
    fn fetch(board: &'a BlackBoard) -> Self {
        (A::fetch(board), B::fetch(board))
    }
}

// Implement for a tuple of 3 items
impl<'a, A, B, C> LintData<'a> for (A, B, C)
where
    A: LintData<'a>,
    B: LintData<'a>,
    C: LintData<'a>,
{
    fn fetch(board: &'a BlackBoard) -> Self {
        (A::fetch(board), B::fetch(board), C::fetch(board))
    }
}
