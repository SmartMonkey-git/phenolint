use crate::tree::pointer::Pointer;

pub trait Spanning {
    fn span(&self, ptr: &Pointer) -> Option<(usize, usize)>;
}
