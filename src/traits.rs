use crate::diagnostics::report::LintReport;
use crate::error::{LintResult, RuleInitError};
use crate::json::PhenopacketCursor;
use crate::linter_context::LinterContext;
use phenopackets::schema::v2::Phenopacket;
use std::borrow::Cow;

pub trait LintRule: RuleCheck + FromContext {
    const RULE_ID: &'static str;
}

pub trait FromContext {
    fn from_context(context: &LinterContext) -> Result<Box<dyn RuleCheck>, RuleInitError>;
}

pub trait RuleCheck {
    fn check(&self, phenostr: &mut PhenopacketCursor, report: &mut LintReport);
}

pub trait IntoBytes {
    fn into_bytes(self) -> Cow<'static, [u8]>;
}

impl IntoBytes for &str {
    fn into_bytes(self) -> Cow<'static, [u8]> {
        Cow::Owned(self.as_bytes().to_vec())
    }
}

impl IntoBytes for String {
    fn into_bytes(self) -> Cow<'static, [u8]> {
        Cow::Owned(self.into_bytes())
    }
}

impl IntoBytes for &[u8] {
    fn into_bytes(self) -> Cow<'static, [u8]> {
        Cow::Owned(self.to_vec())
    }
}

impl IntoBytes for Phenopacket {
    fn into_bytes(self) -> Cow<'static, [u8]> {
        let bytes = serde_json::to_vec(&self).expect("Serializing Phenopacket failed");
        Cow::Owned(serde_json::from_slice(&bytes).expect("Serializing Phenopacket failed"))
    }
}

impl IntoBytes for serde_json::Value {
    fn into_bytes(self) -> Cow<'static, [u8]> {
        let bytes = serde_json::to_vec(&self).expect("Serializing serde_json::Value failed");
        Cow::Owned(bytes)
    }
}
