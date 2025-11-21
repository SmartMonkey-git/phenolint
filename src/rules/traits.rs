use crate::LinterContext;
use crate::blackboard::BlackBoard;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;

pub trait LintRule: RuleFromContext + Send + Sync {
    fn rule_id(&self) -> &str;

    fn check_erased(&self, board: &BlackBoard) -> Vec<LintViolation>;
}

pub trait RuleFromContext {
    fn from_context(context: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError>
    where
        Self: Sized;
}

pub trait RuleCheck: Send + Sync + 'static {
    type Data<'a>: LintData<'a>;
    fn check(&self, data: Self::Data<'_>) -> Vec<LintViolation>;
}

impl<T> LintRule for T
where
    T: RuleCheck + RuleFromContext,
{
    fn rule_id(&self) -> &str {
        self.rule_id()
    }

    fn check_erased(&self, board: &BlackBoard) -> Vec<LintViolation> {
        let data = <Self as RuleCheck>::Data::fetch::<T>(board);

        self.check(data)
    }
}

pub trait LintData<'a> {
    fn fetch<T: 'static>(board: &'a BlackBoard) -> Self
    where
        Self: Sized;
}
