#![allow(dead_code)]
use crate::report::report_registry::ReportRegistry;
use inventory;

pub struct ReportRegistration {
    pub(crate) rule_id: &'static str,
    pub(crate) register: fn(&mut ReportRegistry),
}

inventory::collect!(ReportRegistration);
