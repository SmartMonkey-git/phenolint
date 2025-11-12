use crate::diagnostics::LintReport;
use crate::error::{LintResult, LinterError};
use crate::new::json_traverser::{PhenopacketJsonTraverser, PhenopacketYamlTraverser};
use crate::new::router::NodeRouter;
use crate::new::traverser_factory::TraverserFactory;
use crate::{LinterContext, NodeParser, PhenopacketNodeTraversal};

pub struct Linter;

impl Linter {
    // str for now
    fn lint<T: 'static, P: NodeParser<T>>(
        &self,
        phenobytes: &[u8],
        patch: bool,
        quite: bool,
    ) -> LintResult
    where
        PhenopacketJsonTraverser: PhenopacketNodeTraversal<T>,
        PhenopacketYamlTraverser: PhenopacketNodeTraversal<T>,
    {
        let context = LinterContext::default();
        let mut report = LintReport::default();

        let traverser: Box<dyn PhenopacketNodeTraversal<T>> =
            match TraverserFactory::factory::<T>(phenobytes) {
                Ok(t) => t,
                Err(err) => return LintResult::err(LinterError::InitError(err)),
            };

        for node in traverser.traverse() {
            NodeRouter::<T, P>::route_value(&node, &context, &mut report);
        }

        /*
        if !quite {
            for info in report.findings() {
                if let Err(err) = ReportParser::emit(info.violation().report(), phenostr) {
                    warn!(
                        "Unable to parse and emit report for: '{}'",
                        info.violation().rule_id()
                    );
                };
            }
        }*/

        LintResult::ok(report)
    }
}
