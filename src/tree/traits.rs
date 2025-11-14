use crate::tree::pointer::Pointer;
use crate::tree::span_types::Span;
use dyn_clone::DynClone;

pub trait Spanning: DynClone {
    fn span(&self, ptr: &Pointer) -> Option<Span>;
    fn clone_box(&self) -> Box<dyn Spanning>;
}
dyn_clone::clone_trait_object!(Spanning);
