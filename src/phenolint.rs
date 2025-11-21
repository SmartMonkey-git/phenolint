use crate::LinterContext;
use crate::blackboard::BlackBoard;
use crate::diagnostics::enums::PhenopacketData;
use crate::diagnostics::{LintFinding, LintReport};
use crate::enums::InputTypes;
use crate::error::{InitError, LintResult, LinterError, ParsingError, validation_error_to_string};
use crate::parsing::phenopacket_parser::PhenopacketParser;
use crate::patches::patch_engine::PatchEngine;
use crate::patches::patch_registry::PatchRegistry;
use crate::report::parser::ReportParser;
use crate::report::report_registry::ReportRegistry;
use crate::rules::rule_registry::{RuleRegistry, check_duplicate_rule_ids};
use crate::schema_validation::validator::PhenopacketSchemaValidator;
use crate::supplier::NodeSupplier;
use crate::traits::Lint;
use crate::tree::abstract_pheno_tree::AbstractTreeTraversal;
use crate::tree::node::{DynamicNode, MaterializedNode};
use crate::tree::pointer::Pointer;
use log::{error, warn};
use phenopackets::schema::v2::Phenopacket;
use prost::Message;
use serde_json::Value;
use std::any::TypeId;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct Phenolint {
    rule_registry: RuleRegistry,
    patch_registry: PatchRegistry,
    report_registry: ReportRegistry,
    supplier: NodeSupplier,
    patch_engine: PatchEngine,
    validator: PhenopacketSchemaValidator,
}

impl Phenolint {
    pub fn new(context: LinterContext, rule_ids: Vec<String>) -> Self {
        check_duplicate_rule_ids();

        let rule_registry = RuleRegistry::with_enabled_rules(rule_ids.as_slice(), &context);
        let report_registry = ReportRegistry::with_enabled_reports(rule_ids.as_slice(), &context);
        let patch_registry = PatchRegistry::with_enabled_patches(rule_ids.as_slice(), &context);

        Phenolint {
            rule_registry,
            report_registry,
            patch_registry,
            supplier: NodeSupplier,
            patch_engine: PatchEngine,
            validator: PhenopacketSchemaValidator::default(),
        }
    }

    fn emit(phenostr: &str, report: &LintReport) {
        for (info, report_specs) in report
            .findings()
            .iter()
            .filter_map(|info| info.report().map(|rs| (info, rs)))
        {
            if ReportParser::emit(report_specs, phenostr).is_err() {
                warn!(
                    "Unable to parse and emit report for: '{}'",
                    info.violation().rule_id()
                );
            }
        }
    }
}

impl Lint<str> for Phenolint {
    fn lint(&mut self, phenostr: &str, patch: bool, quit: bool) -> LintResult {
        let mut report = LintReport::default();

        let (values, spans, input_type) = match PhenopacketParser::to_abstract_tree(phenostr) {
            Ok((values, spans, input_type)) => (values, spans, input_type),
            Err(err) => return LintResult::err(LinterError::ParsingError(err)),
        };

        if let Err(err) = self.validator.validate_phenopacket(&values) {
            return LintResult::err(LinterError::InvalidPhenopacket {
                path: err.instance_path.to_string(),
                reason: validation_error_to_string(&err.kind),
            });
        }

        let apt = AbstractTreeTraversal::new(&values, &spans);
        let mut blackboards: BlackBoard = BlackBoard::new();

        for node in apt.traverse() {
            self.supplier.supply_rules(&node, &mut blackboards)
        }

        let root_node = DynamicNode {
            value: values.clone(),
            spans,
            pointer: Pointer::at_root(),
        };

        let mut findings = vec![];
        for rule in self.rule_registry.rules() {
            let violations = rule.check_erased(&blackboards);

            for violation in violations {
                let patches =
                    self.patch_registry
                        .get_patches_for(rule.rule_id(), &root_node, &violation);

                let report =
                    self.report_registry
                        .get_report_for(rule.rule_id(), &root_node, &violation);

                findings.push(LintFinding::new(violation, report, patches));
            }
        }
        report.extend_finding(findings);

        if !quit {
            Self::emit(phenostr, &report);
        }

        if patch & report.has_patches() {
            match self.patch_engine.patch(&values, report.patches()) {
                Ok(patched_phenopacket) => {
                    match convert_phenopacket_to_input_type_str(&patched_phenopacket, input_type) {
                        Ok(phenostr) => {
                            report.patched_phenopacket = Some(phenostr);
                        }
                        Err(err) => {
                            return LintResult::partial(report, LinterError::ParsingError(err));
                        }
                    }
                }
                Err(err) => {
                    return LintResult::partial(report, LinterError::PatchingError(err));
                }
            };
        }

        LintResult::ok(report)
    }
}

impl Lint<PathBuf> for Phenolint {
    fn lint(&mut self, phenopath: &PathBuf, patch: bool, quit: bool) -> LintResult {
        let phenodata = match fs::read(phenopath) {
            Ok(phenodata) => phenodata,
            Err(err) => {
                return LintResult::err(LinterError::InitError(InitError::IO(err)));
            }
        };

        self.lint(phenodata.as_slice(), patch, quit)
    }
}

impl Lint<[u8]> for Phenolint {
    fn lint(&mut self, phenodata: &[u8], patch: bool, quit: bool) -> LintResult {
        let (phenostr, input_type) = match PhenopacketParser::to_string(phenodata) {
            Ok(phenostr) => phenostr,
            Err(err) => {
                return LintResult::err(LinterError::ParsingError(err));
            }
        };

        let mut lint_result = self.lint(phenostr.as_str(), patch, quit);

        convert_phenopacket_to_input_type_u8(&mut lint_result, input_type);

        lint_result
    }
}

fn convert_phenopacket_to_input_type_str(
    patched_phenopacket: &Value,
    input_type: InputTypes,
) -> Result<PhenopacketData, ParsingError> {
    match input_type {
        InputTypes::Json | InputTypes::Protobuf => {
            match serde_json::to_string_pretty(&patched_phenopacket) {
                Ok(patched_phenostr) => Ok(PhenopacketData::Text(patched_phenostr)),
                Err(err) => Err(ParsingError::JsonError(err)),
            }
        }
        InputTypes::Yaml => match serde_yaml::to_string(&patched_phenopacket) {
            Ok(patched_phenostr) => Ok(PhenopacketData::Text(patched_phenostr)),
            Err(err) => Err(ParsingError::YamlError(err)),
        },
    }
}

fn convert_phenopacket_to_input_type_u8(lint_result: &mut LintResult, input_type: InputTypes) {
    if let Some(patched_phenopacket) = lint_result.report.patched_phenopacket.take() {
        let new_data = match patched_phenopacket {
            PhenopacketData::Text(phenotext) => match input_type {
                InputTypes::Protobuf => {
                    let phenopb: Result<Phenopacket, _> = serde_json::from_str(&phenotext);

                    match phenopb {
                        Ok(parsed) => {
                            let mut buf = Vec::new();
                            if let Err(e) = parsed.encode(&mut buf) {
                                error!("Error encoding Phenopacket to protobuf: {:?}", e);
                                lint_result.error =
                                    Some(LinterError::ParsingError(ParsingError::EncodeError(e)));
                                PhenopacketData::Text(phenotext)
                            } else {
                                PhenopacketData::Binary(buf)
                            }
                        }
                        Err(e) => {
                            error!("Error decoding Phenopacket from JSON: {:?}", e);
                            lint_result.error =
                                Some(LinterError::ParsingError(ParsingError::JsonError(e)));
                            PhenopacketData::Text(phenotext)
                        }
                    }
                }
                _ => PhenopacketData::Binary(phenotext.as_bytes().to_vec()),
            },
            PhenopacketData::Binary(phenobytes) => PhenopacketData::Binary(phenobytes),
        };

        lint_result.report.patched_phenopacket = Some(new_data);
    }
}
