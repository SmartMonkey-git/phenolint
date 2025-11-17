#![allow(unused)]

use crate::LinterContext;
use crate::diagnostics::LintReport;
use crate::diagnostics::enums::PhenopacketData;
use crate::enums::InputTypes;
use crate::error::{InitError, LintResult, LinterError, ParsingError};
use crate::parsing::phenopacket_parser::PhenopacketParser;
use crate::patches::error::PatchingError;
use crate::patches::patch::Patch;
use crate::patches::patch_engine::PatchEngine;
use crate::patches::patch_registry::PatchRegistry;
use crate::report::parser::ReportParser;
use crate::report::report_registry::ReportRegistry;
use crate::router::NodeRouter;
use crate::traits::Lint;
use crate::tree::abstract_pheno_tree::AbstractTreeTraversal;
use crate::tree::pointer::Pointer;
use codespan_reporting::term::termcolor::Buffer;
use log::{error, warn};
use phenopackets::schema::v2::Phenopacket;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};

pub struct Phenolint {
    context: LinterContext,
    router: NodeRouter,
    patcher: PatchEngine,
}

impl Phenolint {
    fn inner_lint(tree: Value, spans: HashMap<Pointer, Range<usize>>, patch: bool, quit: bool) {}

    pub fn new(context: LinterContext, rule_ids: Vec<String>) -> Self {
        let report_registry = ReportRegistry::with_enabled_reports(rule_ids.as_slice(), &context);
        let patch_registry = PatchRegistry::with_enabled_patches(rule_ids.as_slice(), &context);

        Phenolint {
            context,
            router: NodeRouter::new(rule_ids, report_registry, patch_registry),
            patcher: PatchEngine,
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

    fn patch(
        mut values: &mut Value,
        patches: Vec<&Patch>,
        input_types: &InputTypes,
    ) -> Result<Value, PatchingError> {
        let patched_pp = PatchEngine.patch(values, patches)?;

        Ok(patched_pp)
    }
}

impl Lint<str> for Phenolint {
    fn lint(&mut self, phenostr: &str, patch: bool, quit: bool) -> LintResult {
        let mut report = LintReport::default();

        let (mut values, spans, input_type) = match PhenopacketParser::to_abstract_tree(phenostr) {
            Ok((values, spans, input_type)) => (values, spans, input_type),
            Err(err) => return LintResult::err(LinterError::ParsingError(err)),
        };

        let apt = AbstractTreeTraversal::new(&values, &spans);
        for node in apt.traverse() {
            let findings = self.router.lint_node(&node, &mut self.context);
            report.extend_finding(findings);
        }

        if !quit {
            Self::emit(phenostr, &report);
        }

        if patch && report.has_patches() {
            match Self::patch(&mut values, report.patches(), &input_type) {
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

        convert_phenopacket_to_input_type_u8(&mut lint_result, phenostr.as_str(), input_type);

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

fn convert_phenopacket_to_input_type_u8(
    lint_result: &mut LintResult,
    phenostr: &str,
    input_type: InputTypes,
) {
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
