use crate::error::LintResult;

pub trait Lint<T: ?Sized> {
    fn lint(&self, phenodata: &T, patch: bool, quit: bool) -> LintResult;
}
