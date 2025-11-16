#![allow(dead_code)]
use crate::LinterContext;
use crate::error::FromContextError;
use crate::report::traits::RegisterableReport;
use inventory;

pub struct ReportRegistration {
    pub(crate) rule_id: &'static str,
    pub(crate) factory:
        fn(context: &LinterContext) -> Result<Box<dyn RegisterableReport>, FromContextError>,
}

inventory::collect!(ReportRegistration);
