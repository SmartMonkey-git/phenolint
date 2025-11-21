use crate::rules::traits::LintData;
use crate::tree::node::MaterializedNode;
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct BlackBoard {
    board: HashMap<TypeId, Vec<MaterializedNode>>,
}

impl BlackBoard {
    pub fn new() -> BlackBoard {
        BlackBoard {
            board: HashMap::new(),
        }
    }

    fn get_raw<T: 'static>(&self) -> &[MaterializedNode] {
        if let Some(vec) = self.board.get(&TypeId::of::<T>()) {
            // SAFETY: We only put T in the slot for TypeId::of::<T>
            vec
        } else {
            &[]
        }
    }

    pub fn insert<T: 'static>(&mut self, node: MaterializedNode) {
        self.board.entry(TypeId::of::<T>()).or_default().push(node);
    }
}

pub struct List<'a>(pub &'a [MaterializedNode]);

impl<'a> LintData<'a> for List<'a> {
    fn fetch<T: 'static>(board: &'a BlackBoard) -> Self {
        List(board.get_raw::<T>())
    }
}

impl<'a, A, B> LintData<'a> for (A, B)
where
    A: LintData<'a> + 'static,
    B: LintData<'a> + 'static,
{
    fn fetch<T>(board: &'a BlackBoard) -> Self {
        (A::fetch::<A>(board), B::fetch::<B>(board))
    }
}

// Implement for a tuple of 3 items
impl<'a, A, B, C> LintData<'a> for (A, B, C)
where
    A: LintData<'a> + 'static,
    B: LintData<'a> + 'static,
    C: LintData<'a> + 'static,
{
    fn fetch<T>(board: &'a BlackBoard) -> Self {
        (
            A::fetch::<A>(board),
            B::fetch::<B>(board),
            C::fetch::<C>(board),
        )
    }
}
