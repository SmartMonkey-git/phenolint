use crate::linting_report::LintReport;
use phenopackets::schema::v2::Phenopacket;


pub trait LintRule: RuleCheck + Default  {
    const RULE_ID: &'static str;
}


pub trait RuleCheck {

    fn check(&self, raw_phenopacket: &json_spanned_value::Value, report: &mut LintReport);

}

pub(crate) trait Lint<T> {
    fn lint(&mut self, input: T, fix: bool) -> LintReport;
}
