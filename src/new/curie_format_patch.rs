use crate::CompilePatch;
use crate::enums::PatchAction;
use crate::new::json_traverser::BoxedNode;

struct CurieFormatPatch;

impl<T> CompilePatch<T> for CurieFormatPatch {
    fn compile_patch(&self, node: &BoxedNode<T>) -> PatchAction {
        todo!()
    }
}
