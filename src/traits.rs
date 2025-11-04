use crate::linting_report::LintReport;
use phenopackets::schema::v2::Phenopacket;


// TODO: + Default
pub trait LintRule: RuleCheck + Default  {
    const RULE_ID: &'static str;
}


pub trait RuleCheck {

    fn check(&self, phenopacket: &Phenopacket, report: &mut LintReport);

}

pub(crate) trait Lint<T> {
    fn lint(&'_ mut self, input: T, fix: bool) -> LintReport<'_>;
}
