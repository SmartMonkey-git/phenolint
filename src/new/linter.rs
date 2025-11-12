use crate::diagnostics::{LintReport, ReportParser};
use crate::error::{LintResult, LinterError};
use crate::new::deserializer::PhenopacketsDeserializer;
use crate::new::json_traverser::{PhenopacketJsonTraverser, PhenopacketYamlTraverser};
use crate::new::router::NodeRouter;
use crate::new::traverser_factory::TraverserFactory;
use crate::{DeserializePhenopackets, LinterContext, NodeParser, PhenopacketNodeTraversal};
use log::warn;
use serde_json::Value;

pub struct Linter {
    context: LinterContext,
}

impl Linter {
    fn lint<T, P: NodeParser<T>>(&self, phenobytes: &[u8], patch: bool, quite: bool) -> LintResult
    where
        PhenopacketJsonTraverser: PhenopacketNodeTraversal<T>,
        PhenopacketYamlTraverser: PhenopacketNodeTraversal<T>,
        PhenopacketsDeserializer: DeserializePhenopackets<T>,
    {
        let mut report = LintReport::default();

        let deserialized =
            <PhenopacketsDeserializer as DeserializePhenopackets<T>>::deserialize(phenobytes);

        let traverser: Box<dyn PhenopacketNodeTraversal<T>> =
            match TraverserFactory::factory::<T>(phenobytes) {
                Ok(t) => t,
                Err(err) => return LintResult::err(LinterError::InitError(err)),
            };

        for node in traverser.traverse() {
            NodeRouter::<T, P>::route_value(&node, &self.context, &mut report);
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
