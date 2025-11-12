use crate::PhenopacketNodeTraversal;
use crate::error::InitError;
use crate::new::json_traverser::{PhenopacketJsonTraverser, PhenopacketYamlTraverser};
pub struct TraverserFactory;

impl TraverserFactory {
    pub(crate) fn factory<T>(
        phenobytes: &[u8],
    ) -> Result<Box<dyn PhenopacketNodeTraversal<T>>, InitError>
    where
        PhenopacketJsonTraverser: PhenopacketNodeTraversal<T>,
        PhenopacketYamlTraverser: PhenopacketNodeTraversal<T>,
    {
        // Lots of other Traversers will be instantiated here. There is only one here for example.
        Ok(Box::new(PhenopacketJsonTraverser::new(phenobytes)?))
    }
}
