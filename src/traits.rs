use crate::error::LintResult;

pub trait Lint<T: ?Sized> {
    fn lint(&mut self, phenodata: &T, patch: bool, quit: bool) -> LintResult;
}
