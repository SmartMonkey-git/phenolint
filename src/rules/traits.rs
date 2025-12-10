use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::tree::btree_node_repository::BTreeNodeRepository;
use crate::tree::querying::traits::QueryStrategy;

pub trait LintRule: Send + Sync {
    fn rule_id(&self) -> &str;

    // Needs to be concrete type, because NodeRepository trait is not dyn compatible :(
    fn check_erased(&self, board: &BTreeNodeRepository) -> Vec<LintViolation>;
}

pub trait RuleMetaData {
    fn rule_id(&self) -> &str;
}

pub trait RuleFromContext {
    fn from_context(context: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError>
    where
        Self: Sized;
}

pub trait RuleCheck: Send + Sync + 'static {
    type Query: QueryStrategy;
    fn check(&self, data: <Self::Query as QueryStrategy>::Output) -> Vec<LintViolation>;
}

impl<T> LintRule for T
where
    T: RuleCheck + RuleFromContext + RuleMetaData,
    for<'a> <T as RuleCheck>::Query: Sized,
{
    fn rule_id(&self) -> &str {
        self.rule_id()
    }

    fn check_erased(&self, board: &BTreeNodeRepository) -> Vec<LintViolation> {
        let data = <Self as RuleCheck>::Query::query(board);

        self.check(data)
    }
}
