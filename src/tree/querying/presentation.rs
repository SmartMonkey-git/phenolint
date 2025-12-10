use crate::tree::node::MaterializedNode;
use crate::tree::querying::traits::QueryPresentation;
use std::ops::Deref;

pub struct Flattened<NodeType>(pub Vec<MaterializedNode<NodeType>>);

impl<NodeType> Deref for Flattened<NodeType> {
    type Target = Vec<MaterializedNode<NodeType>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<NodeType> QueryPresentation<Vec<MaterializedNode<NodeType>>> for Flattened<NodeType> {
    fn present(query_res: Vec<MaterializedNode<NodeType>>) -> Self {
        Flattened(query_res)
    }
}

impl<NodeType> QueryPresentation<Vec<Vec<MaterializedNode<NodeType>>>> for Flattened<NodeType> {
    fn present(query_res: Vec<Vec<MaterializedNode<NodeType>>>) -> Self {
        Flattened(query_res.into_iter().flatten().collect())
    }
}

pub struct First<NodeType>(pub Option<MaterializedNode<NodeType>>);

impl<NodeType> Deref for First<NodeType> {
    type Target = Option<MaterializedNode<NodeType>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<NodeType: Clone> QueryPresentation<Vec<MaterializedNode<NodeType>>> for First<NodeType> {
    fn present(query_res: Vec<MaterializedNode<NodeType>>) -> Self {
        First(query_res.first().cloned())
    }
}

impl<NodeType: Clone> QueryPresentation<Vec<Vec<MaterializedNode<NodeType>>>> for First<NodeType> {
    fn present(query_res: Vec<Vec<MaterializedNode<NodeType>>>) -> Self {
        for i in query_res {
            if let Some(j) = i.into_iter().next() {
                return First(Some(j));
            }
        }

        First(None)
    }
}

pub struct Grouped<NodeType>(pub Vec<Vec<MaterializedNode<NodeType>>>);

impl<NodeType> Deref for Grouped<NodeType> {
    type Target = Vec<Vec<MaterializedNode<NodeType>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<NodeType: Clone> QueryPresentation<Vec<Vec<MaterializedNode<NodeType>>>>
    for Grouped<NodeType>
{
    fn present(query_res: Vec<Vec<MaterializedNode<NodeType>>>) -> Self {
        Grouped(query_res)
    }
}
