use crate::diagnostics::specs::{DiagnosticSpec, LabelSpecs};
use crate::diagnostics::{LintFinding, LintViolation, ReportSpecs};
use crate::error::RuleInitError;
use crate::json::{JsonCursor, Pointer};
use crate::linter_context::LinterContext;
use crate::register_rule;
use crate::rules::rule_registry::{BoxedRuleCheck, LintingPolicy};
use crate::traits::{LintRule, RuleCheck, RuleFromContext};
use codespan_reporting::diagnostic::{LabelStyle, Severity};
use phenolint_macros::register_rule as rr;
use phenopackets::schema::v2::core::OntologyClass;
use regex::Regex;

#[derive(Debug)]
#[rr(id = "CURIE001")]
pub struct CurieFormatRule {
    regex: Regex,
}

impl RuleFromContext for CurieFormatRule {
    type CheckType = OntologyClass;

    fn from_context(_: &LinterContext) -> Result<BoxedRuleCheck<OntologyClass>, RuleInitError> {
        Ok(Box::new(CurieFormatRule {
            regex: Regex::new("^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$").expect("Invalid regex"),
        }))
    }
}

impl RuleCheck for CurieFormatRule {
    type CheckType = OntologyClass;

    fn check(&self, node: &OntologyClass) -> Vec<LintViolation> {
        println!("{}", Self::RULE_ID);
        println!("{:?}", node);

        vec![LintViolation::new(
            Self::RULE_ID,
            Pointer::new("phenofeature/4/type/id"),
        )]
    }
}

impl CurieFormatRule {
    fn write_report(cursor: &mut JsonCursor) -> ReportSpecs {
        cursor.push_anchor();
        let (curie_start, curie_end) = cursor.down("id").span().expect("Should have found span");
        cursor.up();

        let (context_span_start, context_span_end) =
            cursor.up().span().expect("Should have found span");

        cursor.up().up();
        if let Some(val) = cursor.current_value()
            && val.as_object().is_some()
        {
            cursor.up();
        };
        let (label_start, label_end) = cursor.span().expect("Should have found span");

        let labels = vec![
            LabelSpecs {
                style: LabelStyle::Primary,
                range: curie_start..curie_end,
                message: "Expected CURIE with format CURIE:12345".to_string(),
            },
            LabelSpecs {
                style: LabelStyle::Secondary,
                range: context_span_start..context_span_end,
                message: "For this Ontology Class".to_string(),
            },
            LabelSpecs {
                style: LabelStyle::Secondary,
                range: label_start..label_end,
                message: "In this section".to_string(),
            },
        ];

        let diagnostic_spec = DiagnosticSpec {
            severity: Severity::Error,
            code: Self::RULE_ID.to_string(),
            message: "CURIE formatted incorrectly".to_string(),
            labels,
            notes: vec![
                "Note: All CURIE IDs need to follow the format: ^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$"
                    .to_string(),
            ],
        };

        ReportSpecs::new(diagnostic_spec)
    }
}
